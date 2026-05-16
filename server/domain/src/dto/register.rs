use super::decode;
use crate::models::{Email, PlainPassword, Username};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use pipeline::{
    error::PipelineResult,
    primitives::{IkPub, IkPubEd, OtpkPub, SpkPub, SpkPubSig},
    request::Request,
    stages::{Dto, Validated},
};

/// Raw inbound JSON for POST /register
#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub ik_pub: String,
    pub ik_pub_ed: String,
    pub spk_pub: String,
    pub spk_pub_sig: String,
    pub otpks: Vec<String>,
}

/// Validated counterpart: only constructable via `RegisterDto::validate`.
#[derive(Debug)]
pub struct RegisterInput {
    pub username: Username,
    pub email: Email,
    pub password: PlainPassword,
    pub ik_pub: IkPub,
    pub ik_pub_ed: IkPubEd,
    pub spk_pub: SpkPub,
    pub spk_pub_sig: SpkPubSig,
    pub otpks: Vec<OtpkPub>,
}

fn to_validate(dto: RegisterDto) -> PipelineResult<RegisterInput> {
    Ok(RegisterInput {
        username: Username::parse(dto.username)?,
        email: Email::parse(dto.email)?,
        password: PlainPassword::parse(dto.password)?,

        ik_pub: decode(dto.ik_pub)?,
        ik_pub_ed: decode(dto.ik_pub_ed)?,
        spk_pub: decode(dto.spk_pub)?,
        spk_pub_sig: decode(dto.spk_pub_sig)?,

        otpks: dto
            .otpks
            .into_iter()
            .map(decode)
            .collect::<Result<_, _>>()?,
    })
}

trait RegDtoValidator {
    fn validate(self) -> PipelineResult<Request<Validated, RegisterInput>>;
}

impl RegDtoValidator for Request<Dto, RegisterDto> {
    fn validate(self) -> PipelineResult<Request<Validated, RegisterInput>> {
        self.try_advance_with(to_validate)
    }
}

// Ergonomic free function used by the handler.
pub fn validate_register(
    req: Request<Dto, RegisterDto>,
) -> PipelineResult<Request<Validated, RegisterInput>> {
    req.validate()
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub otpk_count: i64,
}
