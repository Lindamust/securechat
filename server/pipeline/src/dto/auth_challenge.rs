use crate::{
    error::PipelineResult,
    primitives::{IkPub, Nonce},
    request::Request,
    stages::{Dto, Validated},
};
use serde::{Deserialize, Serialize};

use validator::Validate;

use super::{ValidateDtoExt, decode};


#[derive(Debug, Deserialize, Validated)]
pub struct AuthChallengeBody {
    pub ik_pub: IkPub,
}

#[derive(Debug, Serialize)]
pub struct AuthChallengeResponse {
    pub nonce: Nonce,
}
