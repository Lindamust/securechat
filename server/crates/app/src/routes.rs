use infra::database::PgDatabase;
use pipeline_core::{
    dto::{
        AuthChallengeBody, AuthChallengeNonce, AuthChallengeResponse, CreatedUser, RegisterBody,
        RegisterResponse,
    },
    engine::Pipeline,
    typestate::{error::PipelineResult, request::Request, stages::Executed},
};

pub fn register_pipeline() -> Pipeline<RegisterBody, RegisterResponse, PgDatabase> {
    Pipeline::new(
        false,
        |req: Request<Executed, CreatedUser>| -> PipelineResult<RegisterResponse> {
            let user = req.into_inner();
            Ok(RegisterResponse {
                id: user.id,
                inserted: user.inserted,
            })
        },
    )
}

pub fn auth_challenge_pipeline() -> Pipeline<AuthChallengeBody, AuthChallengeResponse, PgDatabase> {
    Pipeline::new(
        false,
        |req: Request<Executed, AuthChallengeNonce>| -> PipelineResult<AuthChallengeResponse> {
            let auth_nonce = req.into_inner();
            Ok(AuthChallengeResponse {
                nonce: auth_nonce.nonce,
            })
        },
    )
}
