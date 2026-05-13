use crate::models::{Email, PlainPassword, Username};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use pipeline::{
    error::PipelineResult,
    request::Request,
    stages::{Dto, Validated},
};

/// Raw inbound JSON for POST /register
#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Validated counterpart: only constructable via `RegisterDto::validate`.
#[derive(Debug)]
pub struct RegisterInput {
    pub username: Username,
    pub email: Email,
    pub password: PlainPassword,
}

impl RegisterDto {
    pub fn validate(
        self,
        req: Request<Dto, Self>,
    ) -> PipelineResult<Request<Validated, RegisterInput>> {
        let validated = RegisterInput {
            username: Username::parse(self.username)?,
            email: Email::parse(self.email)?,
            password: PlainPassword::parse(self.password)?,
        };
        Ok(req.advance(validated))
    }
}

// Ergonomic free function used by the handler.
pub fn validate_register(
    req: Request<Dto, RegisterDto>,
) -> PipelineResult<Request<Validated, RegisterInput>> {
    let dto = req.into_inner();
    let validated = RegisterInput {
        username: Username::parse(dto.username)?,
        email: Email::parse(dto.email)?,
        password: PlainPassword::parse(dto.password)?,
    };
    Ok(Request::new(validated))
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub username: String,
}
