use crate::models::{Email, PlainPassword, Username};
use serde::{Deserialize, Serialize, de::IntoDeserializer};
use uuid::Uuid;

use pipeline::{
    error::PipelineResult, primitives::{IkPub, IkPubEd, OtpkPub, SpkPub, SpkPubSig}, request::Request, stages::{Dto, Validated}
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

impl RegisterDto {
    pub fn validate(
        self,
        req: Request<Dto, Self>,
    ) -> PipelineResult<Request<Validated, RegisterInput>> {
        let otpks = self
            .otpks
            .into_iter()
            .map(|s| OtpkPub::deserialize(s.into_deserializer()))
            .collect::<Result<Vec<_>, _>>()?;


        let validated = RegisterInput {
            username: Username::parse(self.username)?,
            email: Email::parse(self.email)?,
            password: PlainPassword::parse(self.password)?,
            ik_pub: IkPub::deserialize(self.ik_pub.into_deserializer())?,
            ik_pub_ed: IkPubEd::deserialize(self.ik_pub_ed.into_deserializer())?,
            spk_pub: SpkPub::deserialize(self.spk_pub.into_deserializer())?,
            spk_pub_sig: SpkPubSig::deserialize(self.spk_pub_sig.into_deserializer())?,
            otpks
        };
        Ok(req.advance(validated))
    }
}

// Ergonomic free function used by the handler.
pub fn validate_register(
    req: Request<Dto, RegisterDto>,
) -> PipelineResult<Request<Validated, RegisterInput>> {
    let dto = req.into_inner();
    let validated = RegisterInput {
        username: Username::parse(dto.username)?,
        email: Email::parse(dto.email)?,
        password: PlainPassword::parse(dto.password)?,
        ik_pub: IkPub::deserialize(dto.ik_pub.into_deserializer())?,
        ik_pub_ed: IkPubEd::deserialize(dto.ik_pub_ed.into_deserializer())?,
        spk_pub: SpkPub::deserialize(dto.spk_pub.into_deserializer())?,
        spk_pub_sig: SpkPubSig::deserialize(dto.spk_pub_sig.into_deserializer())?,
        otpks: Vec::new().into()
    };
    Ok(Request::new(validated))
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub username: String,
}
