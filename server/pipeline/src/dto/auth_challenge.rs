use crate::{
    primitives::{IkPub, Nonce},
};
use serde::{Deserialize, Serialize};

use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AuthChallengeBody {
    pub ik_pub: IkPub,
}

#[derive(Debug, Serialize)]
pub struct AuthChallengeResponse {
    pub nonce: Nonce,
}
