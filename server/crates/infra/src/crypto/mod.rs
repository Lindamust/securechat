use pipeline_core::primitives::ReprBytes;

use ed25519_dalek::{Signature, VerifyingKey};
use pipeline_core::primitives::{IkPub, Nonce, SigData};

pub struct CryptoManager;

pub fn verify_signed_nonce(ik: IkPub, nonce: Nonce, sig: SigData) -> bool {
    let Ok(public_key) = VerifyingKey::from_bytes(&ik.bytes_const()) else {
        return false;
    };

    let signature = Signature::from_bytes(&sig.bytes_const());

    public_key
        .verify_strict(nonce.0.0.as_ref(), &signature)
        .is_ok()
}
