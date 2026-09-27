# crypto

## Classs

- [AeadCipher](AeadCipher.md) — Authenticated AEAD Cipher for Mesh Packets
- [DeviceIdentityKey](DeviceIdentityKey.md) — Long-term Device Identity Key Pair (Ed25519)
- [DeviceIdentityPublicKey](DeviceIdentityPublicKey.md) — Public component of Device Identity Key (Ed25519)
- [DeviceSignature](DeviceSignature.md) — 64-byte Ed25519 signature
- [EphemeralHandshake](EphemeralHandshake.md) — Ephemeral P2P Handshake state
- [SessionKey](SessionKey.md) — Ephemeral 256-bit symmetric session key derived via Blake3 KDF

## Functions

- [accept_peer_nonce](accept_peer_nonce.md)
- [accept_peer_nonce](accept_peer_nonce_1.md)
- [as_bytes](as_bytes.md)
- [as_bytes](as_bytes_1.md)
- [as_bytes](as_bytes_2.md)
- [as_bytes](as_bytes_3.md)
- [as_bytes](as_bytes_4.md)
- [as_bytes](as_bytes_5.md)
- [decode](decode.md)
- [decrypt](decrypt.md) — Decrypts ciphertext packet and validates authentication tag
- [decrypt](decrypt_1.md) — Decrypts ciphertext packet and validates authentication tag
- [default](default.md)
- [default](default_1.md)
- [derive_session_keys](derive_session_keys.md)
- [derive_session_keys](derive_session_keys_1.md)
- [deserialize](deserialize.md)
- [deserialize](deserialize_1.md)
- [encode](encode.md)
- [encrypt](encrypt.md) — Encrypts plaintext packet using Blake3 keyed stream cipher with authenticated tag
- [encrypt](encrypt_1.md) — Encrypts plaintext packet using Blake3 keyed stream cipher with authenticated tag
- [fingerprint](fingerprint.md)
- [fingerprint](fingerprint_1.md)
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [fmt](fmt_2.md)
- [fmt](fmt_3.md)
- [from_bytes](from_bytes.md)
- [from_bytes](from_bytes_1.md)
- [from_bytes](from_bytes_2.md)
- [from_bytes](from_bytes_3.md)
- [from_handshake](from_handshake.md)
- [from_handshake](from_handshake_1.md)
- [from_pkcs8](from_pkcs8.md) — Creates key from PKCS8 document bytes
- [from_pkcs8](from_pkcs8_1.md) — Creates key from PKCS8 document bytes
- [generate](generate.md) — Generates a new random Ed25519 device key pair
- [generate](generate_1.md) — Generates a new random Ed25519 device key pair
- [local_nonce](local_nonce.md)
- [local_nonce](local_nonce_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [public_key](public_key.md)
- [public_key](public_key_1.md)
- [serialize](serialize.md)
- [serialize](serialize_1.md)
- [sign](sign.md)
- [sign](sign_1.md)
- [test_aead_cipher_encrypt_decrypt_roundtrip](test_aead_cipher_encrypt_decrypt_roundtrip.md) — [test]
- [test_device_identity_key_sign_and_verify](test_device_identity_key_sign_and_verify.md) — [test]
- [test_signature_serialization](test_signature_serialization.md) — [test]
- [verify](verify.md)
- [verify](verify_1.md)
