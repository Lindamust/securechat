use pipeline::commands::{
    AuthChallengeNonce, CreateAuthCommand, CreatedUser, build_auth_command,
    build_auth_response,
};

use pipeline::commands::{
    build_register_command, RegisterUserCommand,
};

use pipeline::dto::{
    AuthChallengeBody, AuthChallengeDto, AuthChallengeInput, AuthChallengeResponse, RegisterBody, RegisterResponse, validate_auth_challenege
};

use pipeline::{
    auth::Identity,
    engine::Pipeline,
    error::PipelineResult,
    request::Request,
    stages::{CommandReady, Executed, Validated},
};

pub fn register_pipeline()
-> Pipeline<RegisterBody, RegisterUserCommand, RegisterResponse> {
    Pipeline::new(
        false,
        |req: Request<Validated, RegisterInput>, _identity: &Identity| {
            Ok(build_register_command(req))
        },
        |req: Request<Executed, CreatedUser>| -> PipelineResult<RegisterResponse> {
            let user: CreatedUser = req.into_inner();
            Ok( RegisterResponse { id: user.id, inserted: user.inserted })
        }
    )
}

pub fn auth_challenge_pipeline()
-> Pipeline<AuthChallengeBody, CreateAuthCommand, AuthChallengeResponse> {
    Pipeline::new(
        false,
        |req: Request<Validated, AuthChallengeInput>, _identity: &Identity| {
            Ok(build_auth_command(req))
        },
        |req: Request<Executed, AuthChallengeNonce>| -> PipelineResult<AuthChallengeResponse> {
            let auth_nonce = req.into_inner();
            Ok( AuthChallengeResponse { nonce: auth_nonce.nonce })
        }
    )
}
