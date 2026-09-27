---
okf_version: "0.2"
type: Module
title: crypto
description: "# Cryptographic Primitives for Zero-Trust Mesh Network (`crates/oxide-network/src/crypto.rs`)"
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto
language: rust
---

# crypto

# Cryptographic Primitives for Zero-Trust Mesh Network (`crates/oxide-network/src/crypto.rs`)

## Docstring

# Cryptographic Primitives for Zero-Trust Mesh Network (`crates/oxide-network/src/crypto.rs`)

Provides Ed25519 device identity keys, ephemeral session handshakes,
Blake3 HKDF key derivation, and authenticated AEAD encryption/decryption.

## Relationships

| Type | Target |
|------|--------|
| related | [DeviceIdentityKey](/crates/oxide-network/src/crypto/DeviceIdentityKey.md) |
| related | [generate](/crates/oxide-network/src/crypto/generate.md) |
| related | [from_pkcs8](/crates/oxide-network/src/crypto/from_pkcs8.md) |
| related | [public_key](/crates/oxide-network/src/crypto/public_key.md) |
| related | [sign](/crates/oxide-network/src/crypto/sign.md) |
| related | [generate](/crates/oxide-network/src/crypto/generate.md) |
| related | [from_pkcs8](/crates/oxide-network/src/crypto/from_pkcs8.md) |
| related | [public_key](/crates/oxide-network/src/crypto/public_key.md) |
| related | [sign](/crates/oxide-network/src/crypto/sign.md) |
| related | [fmt](/crates/oxide-network/src/crypto/fmt.md) |
| related | [fmt](/crates/oxide-network/src/crypto/fmt.md) |
| related | [DeviceIdentityPublicKey](/crates/oxide-network/src/crypto/DeviceIdentityPublicKey.md) |
| related | [from_bytes](/crates/oxide-network/src/crypto/from_bytes.md) |
| related | [as_bytes](/crates/oxide-network/src/crypto/as_bytes.md) |
| related | [fingerprint](/crates/oxide-network/src/crypto/fingerprint.md) |
| related | [verify](/crates/oxide-network/src/crypto/verify.md) |
| related | [from_bytes](/crates/oxide-network/src/crypto/from_bytes.md) |
| related | [as_bytes](/crates/oxide-network/src/crypto/as_bytes.md) |
| related | [fingerprint](/crates/oxide-network/src/crypto/fingerprint.md) |
| related | [verify](/crates/oxide-network/src/crypto/verify.md) |
| related | [DeviceSignature](/crates/oxide-network/src/crypto/DeviceSignature.md) |
| related | [from_bytes](/crates/oxide-network/src/crypto/from_bytes.md) |
| related | [as_bytes](/crates/oxide-network/src/crypto/as_bytes.md) |
| related | [from_bytes](/crates/oxide-network/src/crypto/from_bytes.md) |
| related | [as_bytes](/crates/oxide-network/src/crypto/as_bytes.md) |
| related | [serialize](/crates/oxide-network/src/crypto/serialize.md) |
| related | [serialize](/crates/oxide-network/src/crypto/serialize.md) |
| related | [deserialize](/crates/oxide-network/src/crypto/deserialize.md) |
| related | [deserialize](/crates/oxide-network/src/crypto/deserialize.md) |
| related | [SessionKey](/crates/oxide-network/src/crypto/SessionKey.md) |
| related | [new](/crates/oxide-network/src/crypto/new.md) |
| related | [from_handshake](/crates/oxide-network/src/crypto/from_handshake.md) |
| related | [as_bytes](/crates/oxide-network/src/crypto/as_bytes.md) |
| related | [new](/crates/oxide-network/src/crypto/new.md) |
| related | [from_handshake](/crates/oxide-network/src/crypto/from_handshake.md) |
| related | [as_bytes](/crates/oxide-network/src/crypto/as_bytes.md) |
| related | [fmt](/crates/oxide-network/src/crypto/fmt.md) |
| related | [fmt](/crates/oxide-network/src/crypto/fmt.md) |
| related | [AeadCipher](/crates/oxide-network/src/crypto/AeadCipher.md) |
| related | [new](/crates/oxide-network/src/crypto/new.md) |
| related | [encrypt](/crates/oxide-network/src/crypto/encrypt.md) |
| related | [decrypt](/crates/oxide-network/src/crypto/decrypt.md) |
| related | [new](/crates/oxide-network/src/crypto/new.md) |
| related | [encrypt](/crates/oxide-network/src/crypto/encrypt.md) |
| related | [decrypt](/crates/oxide-network/src/crypto/decrypt.md) |
| related | [EphemeralHandshake](/crates/oxide-network/src/crypto/EphemeralHandshake.md) |
| related | [default](/crates/oxide-network/src/crypto/default.md) |
| related | [default](/crates/oxide-network/src/crypto/default.md) |
| related | [new](/crates/oxide-network/src/crypto/new.md) |
| related | [local_nonce](/crates/oxide-network/src/crypto/local_nonce.md) |
| related | [accept_peer_nonce](/crates/oxide-network/src/crypto/accept_peer_nonce.md) |
| related | [derive_session_keys](/crates/oxide-network/src/crypto/derive_session_keys.md) |
| related | [new](/crates/oxide-network/src/crypto/new.md) |
| related | [local_nonce](/crates/oxide-network/src/crypto/local_nonce.md) |
| related | [accept_peer_nonce](/crates/oxide-network/src/crypto/accept_peer_nonce.md) |
| related | [derive_session_keys](/crates/oxide-network/src/crypto/derive_session_keys.md) |
| related | [encode](/crates/oxide-network/src/crypto/encode.md) |
| related | [decode](/crates/oxide-network/src/crypto/decode.md) |
| related | [test_device_identity_key_sign_and_verify](/crates/oxide-network/src/crypto/test_device_identity_key_sign_and_verify.md) |
| related | [test_signature_serialization](/crates/oxide-network/src/crypto/test_signature_serialization.md) |
| related | [test_aead_cipher_encrypt_decrypt_roundtrip](/crates/oxide-network/src/crypto/test_aead_cipher_encrypt_decrypt_roundtrip.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
