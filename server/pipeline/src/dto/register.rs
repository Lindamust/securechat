use crate::{
    models::{Email, PlainPassword, Username},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    primitives::{IkPub, IkPubEd, OtpkPub, SpkPub, SpkPubSig},
};

use validator::Validate;

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

