use crate::dto::{AuthChallengeBody, AuthChallengeResponse};
use crate::{
    Command,
    error::PipelineResult,
    primitives::{IkPub, Nonce},
    request::Request,
    stages::{CommandReady, Executed, Validated},
};

#[derive(Debug)]
pub struct CreateAuthCommand {
    pub ik_pub: IkPub,
}

#[derive(Debug)]
pub struct AuthChallengeNonce {
    pub nonce: Nonce,
}

impl Command for CreateAuthCommand {
    type Output = AuthChallengeNonce;
}

pub fn build_auth_command(
    req: Request<Validated, AuthChallengeBody>,
) -> Request<CommandReady, CreateAuthCommand> {
    let input = req.into_inner();
    Request::new(CreateAuthCommand {
        ik_pub: input.ik_pub,
    })
}

pub fn build_auth_response(
    req: Request<Executed, AuthChallengeNonce>,
) -> PipelineResult<AuthChallengeResponse> {
    let auth_nonce = req.into_inner();
    Ok(AuthChallengeResponse {
        nonce: auth_nonce.nonce,
    })
}
