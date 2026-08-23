use http::{Method, Request};
use quinn::Endpoint;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, Error, SignatureScheme};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug)]
struct DummyVerifier;

impl ServerCertVerifier for DummyVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        rustls::crypto::ring::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }
}

#[tokio::test]
async fn test_h3_gateway_integration() -> Result<(), anyhow::Error> {
    // 0. Ensure certs exist for testing
    gateway::ensure_certs();

    // 1. Start in-process H3 server on ephemeral UDP port
    let cfg = Arc::new(common::config::AppConfig::load_default().unwrap());
    let bind_addr: SocketAddr = "127.0.0.1:0".parse()?;
    let (server_addr, _handle) =
        gateway::h3_gateway::start_h3_gateway_on_addr(bind_addr, cfg, None, None).await?;

    // 2. Configure TLS client with custom verifier
    let provider = rustls::crypto::ring::default_provider();
    let mut rustls_config = rustls::ClientConfig::builder_with_provider(Arc::new(provider))
        .with_safe_default_protocol_versions()?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(DummyVerifier))
        .with_no_client_auth();

    rustls_config.alpn_protocols = vec![b"h3".to_vec()];

    let client_config = quinn::ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(rustls_config)?,
    ));

    // 3. Setup client endpoint
    let mut endpoint = Endpoint::client("127.0.0.1:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    // 4. Connect to the ephemeral HTTP/3 gateway
    let conn = endpoint.connect(server_addr, "localhost")?.await?;

    // 5. Establish H3 connection
    let h3_conn = h3_quinn::Connection::new(conn);
    let (mut driver, mut send_request) = h3::client::new(h3_conn).await?;

    // Spawn driver task
    tokio::spawn(async move {
        let _ = futures_util::future::poll_fn(|cx| driver.poll_close(cx)).await;
    });

    // 6. Send GET request to /health/live
    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("https://localhost:{}/health/live", server_addr.port()))
        .body(())?;

    let mut stream = send_request.send_request(req).await?;
    stream.finish().await?;

    // 7. Receive response headers and body
    let resp = stream.recv_response().await?;
    assert_eq!(resp.status(), http::StatusCode::OK);

    let mut body_bytes = bytes::BytesMut::new();
    while let Some(mut chunk) = stream.recv_data().await? {
        use bytes::Buf;
        while chunk.has_remaining() {
            let slice = chunk.chunk();
            body_bytes.extend_from_slice(slice);
            chunk.advance(slice.len());
        }
    }

    let body_str = String::from_utf8(body_bytes.to_vec())?;
    assert_eq!(body_str, "Liveness probe OK");

    Ok(())
}

