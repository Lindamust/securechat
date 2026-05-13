use domain::dto::RegisterInput;
use domain::models::{Email, PlainPassword, Username};
use pipeline::{
    Command,
    request::Request,
    stages::{CommandReady, Validated},
};
use uuid::Uuid;

#[derive(Debug)]
pub struct RegisterUserCommand {
    pub username: Username,
    pub email: Email,
    pub password: PlainPassword,
}

#[derive(Debug)]
pub struct CreatedUser {
    pub id: Uuid,
    pub username: String,
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
    })
}
