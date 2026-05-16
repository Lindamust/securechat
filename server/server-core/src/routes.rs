use commands::{CreatedUser, RegisterUserCommand, build_register_command, build_register_response};
use domain::dto::{RegisterDto, RegisterInput, RegisterResponse, validate_register};
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

pub fn _auth_challenge_pipeline() {
    todo!()
}
