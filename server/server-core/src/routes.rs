use pipeline::commands::{
    AuthChallengeNonce, CreateAuthCommand, CreatedUser, RegisterUserCommand, build_auth_command,
    build_auth_response, build_register_command, build_register_response,
};
use pipeline::dto::{
    AuthChallengeDto, AuthChallengeInput, AuthChallengeResponse, RegisterDto, RegisterInput,
    RegisterResponse, validate_auth_challenege, validate_register,
};
use pipeline::{
    auth::Identity,
    engine::Pipeline,
    request::Request,
    stages::{Executed, Validated},
};

pub fn register_pipeline()
-> Pipeline<RegisterDto, RegisterInput, RegisterUserCommand, RegisterResponse> {
    Pipeline::new(
        false,
        validate_register,
        |req: Request<Validated, RegisterInput>, _identity: &Identity| {
            Ok(build_register_command(req))
        },
        |req: Request<Executed, CreatedUser>| build_register_response(req),
    )
}

pub fn auth_challenge_pipeline()
-> Pipeline<AuthChallengeDto, AuthChallengeInput, CreateAuthCommand, AuthChallengeResponse> {
    Pipeline::new(
        false,
        validate_auth_challenege,
        |req: Request<Validated, AuthChallengeInput>, _identity: &Identity| {
            Ok(build_auth_command(req))
        },
        |req: Request<Executed, AuthChallengeNonce>| build_auth_response(req),
    )
}
