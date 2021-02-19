//! Password-Based Encryption Scheme 2 tests

use core::convert::TryFrom;
use hex_literal::hex;
use pkcs5::pbes2;

/// Example `AlgorithmIdentifier`.
///
/// Generated by OpenSSL and extracted from the `pkcs8` crate's
/// `tests/examples/ed25519-priv-enc-v2.der` test vector.
const PBES2_PBKDF2_HMAC_SHA256_AES256_CBC_ALG_ID: &[u8] = &hex!(
    "305706092a864886f70d01050d304a302906092a864886f70d01050c301c0408
     79d982e70df91a8802020800300c06082a864886f70d02090500301d06096086
     4801650304012a0410b2d02d78b2efd9dff694cf8e0af40925"
);

/// Decoding tests
#[test]
fn decode_pbes2_schemes() {
    let params = pkcs5::Scheme::try_from(PBES2_PBKDF2_HMAC_SHA256_AES256_CBC_ALG_ID)
        .ok()
        .and_then(|s| s.pbes2())
        .unwrap();

    let pbkdf2_params = params.kdf.pbkdf2().unwrap();
    assert_eq!(pbkdf2_params.salt, &hex!("79d982e70df91a88"));
    assert_eq!(pbkdf2_params.iteration_count, 2048);
    assert_eq!(pbkdf2_params.key_length, None);
    assert_eq!(pbkdf2_params.prf, pbes2::Pbkdf2Prf::HmacWithSha256);

    match params.encryption {
        pbes2::EncryptionScheme::Aes256Cbc { iv } => {
            assert_eq!(iv, &hex!("b2d02d78b2efd9dff694cf8e0af40925"));
        }
        other => panic!("unexpected encryption scheme: {:?}", other),
    }
}