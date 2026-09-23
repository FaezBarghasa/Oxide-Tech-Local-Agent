use std::sync::Arc;
use rcgen::{CertificateParams, DistinguishedName, KeyPair, PKCS_ECDSA_P256_SHA256};
use ring::rand::SystemRandom;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Certificate generation error: {0}")]
    CertificateGen(#[from] rcgen::Error),
    #[error("TLS configuration error: {0}")]
    TlsConfig(#[from] rustls::Error),
    #[error("Crypto / entropy error: {0}")]
    CryptoError(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// In-memory Ephemeral mTLS Engine
pub struct EphemeralTlsContext {
    pub client_config: Arc<rustls::ClientConfig>,
    pub server_config: Arc<rustls::ServerConfig>,
    pub node_fingerprint: [u8; 32],
}

impl EphemeralTlsContext {
    /// Bootstrap an ephemeral, in-memory mTLS context bound to local system entropy.
    pub fn bootstrap() -> Result<Self, SecurityError> {
        let _rng = SystemRandom::new();
        let key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256)
            .map_err(SecurityError::CertificateGen)?;

        let mut params = CertificateParams::default();
        let mut dn = DistinguishedName::new();
        dn.push(rcgen::DnType::CommonName, "oxide-internal-mesh.local");
        params.distinguished_name = dn;
        params.subject_alt_names = vec![
            rcgen::SanType::DnsName("oxide-internal-mesh.local".to_string().try_into().map_err(|e: rcgen::Error| SecurityError::CryptoError(e.to_string()))?),
            rcgen::SanType::DnsName("localhost".to_string().try_into().map_err(|e: rcgen::Error| SecurityError::CryptoError(e.to_string()))?),
            rcgen::SanType::IpAddress(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)),
        ];

        let cert = params
            .self_signed(&key_pair)
            .map_err(SecurityError::CertificateGen)?;
        let cert_der = cert.der().to_vec();
        let key_der = key_pair.serialize_der();

        // Calculate node fingerprint (SHA-256) over certificate DER
        let fingerprint = ring::digest::digest(&ring::digest::SHA256, &cert_der);
        let mut node_fingerprint = [0u8; 32];
        node_fingerprint.copy_from_slice(fingerprint.as_ref());

        // Install ring crypto provider if not already default
        let _ = rustls::crypto::ring::default_provider().install_default();

        let cert_chain = vec![CertificateDer::from(cert_der.clone())];
        let private_key = PrivateKeyDer::Pkcs8(key_der.into());

        // Configure strict Mutual TLS Server Config
        let mut root_store = rustls::RootCertStore::empty();
        root_store
            .add(CertificateDer::from(cert_der.clone()))
            .map_err(|e| SecurityError::TlsConfig(rustls::Error::General(format!("Failed to add cert to root store: {:?}", e))))?;

        let client_verifier = rustls::server::WebPkiClientVerifier::builder(Arc::new(root_store.clone()))
            .build()
            .map_err(|e| SecurityError::TlsConfig(rustls::Error::General(e.to_string())))?;

        let server_config = rustls::ServerConfig::builder()
            .with_client_cert_verifier(client_verifier)
            .with_single_cert(cert_chain.clone(), private_key.clone_key())
            .map_err(SecurityError::TlsConfig)?;

        // Configure strict Mutual TLS Client Config
        let client_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_client_auth_cert(cert_chain, private_key)
            .map_err(SecurityError::TlsConfig)?;

        Ok(Self {
            client_config: Arc::new(client_config),
            server_config: Arc::new(server_config),
            node_fingerprint,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ephemeral_tls_bootstrap() {
        let ctx = EphemeralTlsContext::bootstrap().expect("Failed to bootstrap EphemeralTlsContext");
        assert_ne!(ctx.node_fingerprint, [0u8; 32]);
        assert_eq!(ctx.node_fingerprint.len(), 32);
    }
}
