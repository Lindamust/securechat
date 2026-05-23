use pipeline::commands::{
    AuthChallengeNonce, CreateAuthCommand, CreatedUser, RegisterUserCommand, build_auth_command,
    build_auth_response, build_register_command, build_register_response,
};

use pipeline::dto::{AuthChallengeBody, AuthChallengeResponse, RegisterBody, RegisterResponse};

use pipeline::{
    auth::Identity,
    engine::Pipeline,
    error::PipelineResult,
    request::Request,
    stages::{CommandReady, Executed, Validated},
};

pub fn register_pipeline() -> Pipeline<RegisterBody, RegisterUserCommand, RegisterResponse> {
    Pipeline::new(
        false,
        |req: Request<Validated, RegisterBody>,
         _identity: &Identity|
         -> PipelineResult<Request<CommandReady, RegisterUserCommand>> {
            Ok(build_register_command(req))
        },
        |req: Request<Executed, CreatedUser>| -> PipelineResult<RegisterResponse> {
            build_register_response(req)
        },
    )
}

pub fn auth_challenge_pipeline()
-> Pipeline<AuthChallengeBody, CreateAuthCommand, AuthChallengeResponse> {
    Pipeline::new(
        false,
        |req: Request<Validated, AuthChallengeBody>,
         _identity: &Identity|
         -> PipelineResult<Request<CommandReady, CreateAuthCommand>> {
            Ok(build_auth_command(req))
        },
        |req: Request<Executed, AuthChallengeNonce>| -> PipelineResult<AuthChallengeResponse> {
            build_auth_response(req)
        },
    )
}
