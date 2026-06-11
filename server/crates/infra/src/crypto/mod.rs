use primitives::ReprBytes;

use domain::models::{IkPub, NonceKey, SigData};
use ed25519_dalek::{Signature, VerifyingKey};

pub struct CryptoManager;

pub fn verify_signed_nonce(ik: &IkPub, nonce: &NonceKey, sig: &SigData) -> bool {
    let Ok(public_key) = VerifyingKey::from_bytes(&ik.bytes_const()) else {
        return false;
    };

    let signature = Signature::from_bytes(&sig.bytes_const());

    public_key
        .verify_strict(nonce.0.0.as_ref(), &signature)
        .is_ok()
}
