//! # Cryptographic Primitives for Zero-Trust Mesh Network (`crates/oxide-network/src/crypto.rs`)
//!
//! Provides Ed25519 device identity keys, ephemeral session handshakes,
//! Blake3 HKDF key derivation, and authenticated AEAD encryption/decryption.

use oxide_core::OxideError;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// 32-byte cryptographic fingerprint of a public key
pub type KeyFingerprint = [u8; 32];

/// Long-term Device Identity Key Pair (Ed25519)
pub struct DeviceIdentityKey {
    key_pair: Ed25519KeyPair,
    raw_pk: [u8; 32],
}

impl DeviceIdentityKey {
    /// Generates a new random Ed25519 device key pair
    pub fn generate() -> Result<Self, OxideError> {
        let rng = SystemRandom::new();
        let doc = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| OxideError::SecurityViolation(format!("Failed to generate PKCS8 key: {e}")))?;
        let key_pair = Ed25519KeyPair::from_pkcs8(doc.as_ref())
            .map_err(|e| OxideError::SecurityViolation(format!("Failed to parse PKCS8 key: {e}")))?;

        let mut raw_pk = [0u8; 32];
        raw_pk.copy_from_slice(key_pair.public_key().as_ref());

        Ok(Self { key_pair, raw_pk })
    }

    /// Creates key from PKCS8 document bytes
    pub fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, OxideError> {
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8)
            .map_err(|e| OxideError::SecurityViolation(format!("Invalid PKCS8 key: {e}")))?;
        let mut raw_pk = [0u8; 32];
        raw_pk.copy_from_slice(key_pair.public_key().as_ref());
        Ok(Self { key_pair, raw_pk })
    }

    pub fn public_key(&self) -> DeviceIdentityPublicKey {
        DeviceIdentityPublicKey { raw: self.raw_pk }
    }

    pub fn sign(&self, msg: &[u8]) -> DeviceSignature {
        let sig = self.key_pair.sign(msg);
        let mut raw = [0u8; 64];
        raw.copy_from_slice(sig.as_ref());
        DeviceSignature { raw }
    }
}

impl fmt::Debug for DeviceIdentityKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceIdentityKey")
            .field("public_key", &hex::encode(self.raw_pk))
            .finish()
    }
}

/// Public component of Device Identity Key (Ed25519)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceIdentityPublicKey {
    pub raw: [u8; 32],
}

impl DeviceIdentityPublicKey {
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        Self { raw: *bytes }
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.raw
    }

    pub fn fingerprint(&self) -> KeyFingerprint {
        *blake3::hash(&self.raw).as_bytes()
    }

    pub fn verify(&self, msg: &[u8], sig: &DeviceSignature) -> Result<(), OxideError> {
        let peer_pk = UnparsedPublicKey::new(&ED25519, &self.raw);
        peer_pk
            .verify(msg, &sig.raw)
            .map_err(|_| OxideError::SecurityViolation("Signature verification failed".into()))
    }
}

/// 64-byte Ed25519 signature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceSignature {
    pub raw: [u8; 64],
}

impl DeviceSignature {
    pub fn from_bytes(bytes: &[u8; 64]) -> Self {
        Self { raw: *bytes }
    }

    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.raw
    }
}

impl Serialize for DeviceSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(self.raw))
    }
}

impl<'de> Deserialize<'de> for DeviceSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 64 {
            return Err(serde::de::Error::custom(format!(
                "Invalid signature length: {} (expected 64)",
                bytes.len()
            )));
        }
        let mut raw = [0u8; 64];
        raw.copy_from_slice(&bytes);
        Ok(Self { raw })
    }
}

/// Ephemeral 256-bit symmetric session key derived via Blake3 KDF
#[derive(Clone)]
pub struct SessionKey {
    key: [u8; 32],
}

impl SessionKey {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn from_handshake(shared_secret: &[u8], salt: &[u8], context: &str) -> Self {
        let mut hasher = blake3::Hasher::new_derive_key(context);
        hasher.update(salt);
        hasher.update(shared_secret);
        let output = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(output.as_bytes());
        Self { key }
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.key
    }
}

impl fmt::Debug for SessionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SessionKey([REDACTED])")
    }
}

/// Authenticated AEAD Cipher for Mesh Packets
pub struct AeadCipher {
    session_key: [u8; 32],
}

impl AeadCipher {
    pub fn new(session_key: [u8; 32]) -> Self {
        Self { session_key }
    }

    /// Encrypts plaintext packet using Blake3 keyed stream cipher with authenticated tag
    pub fn encrypt(&self, seq: u64, plaintext: &[u8]) -> Vec<u8> {
        let mut seq_bytes = [0u8; 8];
        seq_bytes.copy_from_slice(&seq.to_le_bytes());

        let mut hasher = blake3::Hasher::new_keyed(&self.session_key);
        hasher.update(&seq_bytes);
        hasher.update(b"oxide-mesh-datagram-enc");
        let derived_key = hasher.finalize();

        let mut output = Vec::with_capacity(plaintext.len() + 32);
        let stream_hasher = blake3::Hasher::new_keyed(derived_key.as_bytes());
        let mut stream_output = stream_hasher.finalize_xof();

        let mut ciphertext = vec![0u8; plaintext.len()];
        let mut keystream = vec![0u8; plaintext.len()];
        stream_output.fill(&mut keystream);

        for i in 0..plaintext.len() {
            ciphertext[i] = plaintext[i] ^ keystream[i];
        }

        // Authenticated MAC tag
        let mut tag_hasher = blake3::Hasher::new_keyed(&self.session_key);
        tag_hasher.update(&seq_bytes);
        tag_hasher.update(&ciphertext);
        let tag = tag_hasher.finalize();

        output.extend_from_slice(&ciphertext);
        output.extend_from_slice(tag.as_bytes());
        output
    }

    /// Decrypts ciphertext packet and validates authentication tag
    pub fn decrypt(&self, seq: u64, ciphertext_with_tag: &[u8]) -> Result<Vec<u8>, OxideError> {
        if ciphertext_with_tag.len() < 32 {
            return Err(OxideError::SecurityViolation(
                "Ciphertext too short for auth tag".into(),
            ));
        }

        let cipher_len = ciphertext_with_tag.len() - 32;
        let ciphertext = &ciphertext_with_tag[..cipher_len];
        let tag = &ciphertext_with_tag[cipher_len..];

        let mut seq_bytes = [0u8; 8];
        seq_bytes.copy_from_slice(&seq.to_le_bytes());

        let mut tag_hasher = blake3::Hasher::new_keyed(&self.session_key);
        tag_hasher.update(&seq_bytes);
        tag_hasher.update(ciphertext);
        let expected_tag = tag_hasher.finalize();

        if tag != expected_tag.as_bytes() {
            return Err(OxideError::SecurityViolation(
                "AEAD authentication tag mismatch".into(),
            ));
        }

        let mut hasher = blake3::Hasher::new_keyed(&self.session_key);
        hasher.update(&seq_bytes);
        hasher.update(b"oxide-mesh-datagram-enc");
        let derived_key = hasher.finalize();

        let stream_hasher = blake3::Hasher::new_keyed(derived_key.as_bytes());
        let mut stream_output = stream_hasher.finalize_xof();

        let mut plaintext = vec![0u8; ciphertext.len()];
        let mut keystream = vec![0u8; ciphertext.len()];
        stream_output.fill(&mut keystream);

        for i in 0..ciphertext.len() {
            plaintext[i] = ciphertext[i] ^ keystream[i];
        }

        Ok(plaintext)
    }
}

/// Ephemeral P2P Handshake state
pub struct EphemeralHandshake {
    local_nonce: [u8; 32],
    peer_nonce: Option<[u8; 32]>,
}

impl Default for EphemeralHandshake {
    fn default() -> Self {
        Self::new()
    }
}

impl EphemeralHandshake {
    pub fn new() -> Self {
        let rng = SystemRandom::new();
        let mut local_nonce = [0u8; 32];
        rng.fill(&mut local_nonce).expect("SystemRandom failed");
        Self {
            local_nonce,
            peer_nonce: None,
        }
    }

    pub fn local_nonce(&self) -> &[u8; 32] {
        &self.local_nonce
    }

    pub fn accept_peer_nonce(&mut self, peer_nonce: &[u8; 32]) {
        self.peer_nonce = Some(*peer_nonce);
    }

    pub fn derive_session_keys(
        &self,
        local_priv: &DeviceIdentityKey,
        peer_pub: &DeviceIdentityPublicKey,
    ) -> Result<(SessionKey, SessionKey), OxideError> {
        let peer_nonce = self
            .peer_nonce
            .ok_or_else(|| OxideError::SecurityViolation("Peer nonce not yet established".into()))?;

        let mut hasher = blake3::Hasher::new_derive_key("oxide-mesh-handshake-v1");
        hasher.update(&local_priv.public_key().raw);
        hasher.update(&peer_pub.raw);
        hasher.update(&self.local_nonce);
        hasher.update(&peer_nonce);
        let root_secret = hasher.finalize();

        let tx_key = SessionKey::from_handshake(root_secret.as_bytes(), &self.local_nonce, "oxide-tx-session");
        let rx_key = SessionKey::from_handshake(root_secret.as_bytes(), &peer_nonce, "oxide-rx-session");

        Ok((tx_key, rx_key))
    }
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    }

    pub fn decode(hex_str: &str) -> Result<Vec<u8>, String> {
        if !hex_str.len().is_multiple_of(2) {
            return Err("Hex string must have even length".into());
        }
        (0..hex_str.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex_str[i..i + 2], 16)
                    .map_err(|e| format!("Invalid hex byte at {i}: {e}"))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_identity_key_sign_and_verify() {
        let dev_key = DeviceIdentityKey::generate().unwrap();
        let pub_key = dev_key.public_key();

        let msg = b"Mesh node enrollment authorization payload";
        let sig = dev_key.sign(msg);

        assert!(pub_key.verify(msg, &sig).is_ok());
        assert!(pub_key.verify(b"tampered message", &sig).is_err());
    }

    #[test]
    fn test_signature_serialization() {
        let dev_key = DeviceIdentityKey::generate().unwrap();
        let sig = dev_key.sign(b"test");
        let serialized = serde_json::to_string(&sig).unwrap();
        let deserialized: DeviceSignature = serde_json::from_str(&serialized).unwrap();
        assert_eq!(sig.raw, deserialized.raw);
    }

    #[test]
    fn test_aead_cipher_encrypt_decrypt_roundtrip() {
        let session_key = [0x55u8; 32];
        let cipher = AeadCipher::new(session_key);

        let plaintext = b"Hello from remote oxide mesh network overlay!";
        let seq = 42;

        let encrypted = cipher.encrypt(seq, plaintext);
        assert_ne!(&encrypted[..plaintext.len()], plaintext);

        let decrypted = cipher.decrypt(seq, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);

        assert!(cipher.decrypt(seq + 1, &encrypted).is_err());
    }
}
