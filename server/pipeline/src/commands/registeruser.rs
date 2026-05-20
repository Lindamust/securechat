use crate::dto::{RegisterInput, RegisterResponse};
use crate::models::{Email, PlainPassword, Username};
use crate::{
    Command,
    error::PipelineResult,
    primitives::{IkPub, IkPubEd, OtpkPub, SpkPub, SpkPubSig},
    request::Request,
    stages::{CommandReady, Executed, Validated},
};
use uuid::Uuid;

#[derive(Debug)]
pub struct RegisterUserCommand {
    pub username: Username,
    pub email: Email,
    pub password: PlainPassword,
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

impl Command for RegisterUserCommand {
    type Output = CreatedUser;
}

pub fn build_register_command(
    req: Request<Validated, RegisterInput>,
) -> Request<CommandReady, RegisterUserCommand> {
    let input = req.into_inner();
    Request::new(RegisterUserCommand {
        username: input.username,
        email: input.email,
        password: input.password,
        ik_pub: input.ik_pub,
        ik_pub_ed: input.ik_pub_ed,
        spk_pub: input.spk_pub,
        spk_pub_sig: input.spk_pub_sig,
        otpks: input.otpks,
    })
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
