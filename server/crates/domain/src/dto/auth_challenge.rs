use crate::models::{IkPub, NonceKey};

use serde::{Deserialize, Serialize};
use validator::Validate;

// HTTP request + response body

#[derive(Debug, Deserialize, Validate)]
pub struct AuthChallengeBody {
    pub ik_pub: IkPub,
}

#[derive(Debug, Serialize)]
pub struct AuthChallengeResponse {
    pub nonce: NonceKey,
}

// Infra layer return type

#[derive(Debug)]
pub struct InsertedNonce {
    pub nonce: NonceKey,
}
