use super::decode;
use crate::{
    dto::ValidateDtoExt,
    models::{Email, PlainPassword, Username},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::PipelineResult,
    primitives::{IkPub, IkPubEd, OtpkPub, SpkPub, SpkPubSig},
    request::Request,
    stages::{Dto, Validated},
};

use validator::Validate;


/// Validated counterpart: only constructable via `RegisterDto::validate`.
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterBody {
    pub username: Username,
    pub email: Email,
    pub password: PlainPassword,
    pub ik_pub: IkPub,
    pub ik_pub_ed: IkPubEd,
    pub spk_pub: SpkPub,
    pub spk_pub_sig: SpkPubSig,
    pub otpks: Vec<OtpkPub>,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub inserted: i64,
}

