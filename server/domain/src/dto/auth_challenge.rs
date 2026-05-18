use pipeline::{error::PipelineResult, primitives::{IkPub, Nonce}, request::Request, stages::{Dto, Validated}};
use serde::{Deserialize, Serialize};

use super::{decode, ValidateDtoExt};

#[derive(Debug, Deserialize)]
pub struct AuthChallengeDto {
    pub ik_pub: String,
}

#[derive(Debug)]
pub struct AuthChallengeInput {
    pub ik_pub: IkPub,
}

#[derive(Debug, Serialize)]
pub struct AuthChallengeResponse {
    pub nonce: Nonce,
}

fn validate_auth_req(dto: AuthChallengeDto) -> PipelineResult<AuthChallengeInput> {
    Ok(AuthChallengeInput { ik_pub: decode(dto.ik_pub)? })
}

pub fn validate_auth_challenege(req: Request<Dto, AuthChallengeDto>) -> PipelineResult<Request<Validated, AuthChallengeInput>> {
    req.validate(validate_auth_req)
}