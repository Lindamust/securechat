use crate::{
    models::{Email, HashedPassword, PlainPassword, Username},
    primitives::{IkPub, IkPubEd, OtpkPub, SpkPub, SpkPubSig},
    traits::{InfraCommand, IntoCommand},
    typestate::{error::PipelineResult, request::Request, stages::Executed},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// HTTP request + response body

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

// Infra layer command + return type

#[derive(Debug)]
pub struct RegisterUserCommand {
    pub username: Username,
    pub email: Email,
    pub hashed_password: HashedPassword,
    pub ik_pub: IkPub,
    pub ik_pub_ed: IkPubEd,
    pub spk_pub: SpkPub,
    pub spk_pub_sig: SpkPubSig,
    pub otpks: Vec<OtpkPub>,
}

#[derive(Debug)]
pub struct CreatedUser {
    pub id: Uuid,
    pub inserted: i64,
}

impl InfraCommand for RegisterUserCommand {
    type Output = CreatedUser;
}

// Request<Validated, I> ---> Command ---> Request<Executed, O>

impl IntoCommand for RegisterBody {
    type Command = RegisterUserCommand;
    fn into_command(self, _idenity: &crate::extractors::auth::Identity) -> Self::Command {
        RegisterUserCommand {
            username: self.username,
            email: self.email,
            hashed_password: self.password.hash(),
            ik_pub: self.ik_pub,
            ik_pub_ed: self.ik_pub_ed,
            spk_pub: self.spk_pub,
            spk_pub_sig: self.spk_pub_sig,
            otpks: self.otpks,
        }
    }
}

pub fn build_register_response(
    req: Request<Executed, CreatedUser>,
) -> PipelineResult<RegisterResponse> {
    let user = req.into_inner();
    Ok(RegisterResponse {
        id: user.id,
        inserted: user.inserted,
    })
}
