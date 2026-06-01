use domain::dto::{
    AuthChallengeBody, AuthChallengeResponse, InsertedNonce, InsertedUser, RegisterBody,
    RegisterResponse,
};
use infra::database::PgDatabase;
use pipeline_core::{error::PipelineResult, request::Request, stages::Executed};
use pipeline_http::engine::Pipeline;

pub fn register_pipeline() -> Pipeline<RegisterBody, RegisterResponse, PgDatabase> {
    Pipeline::new(
        false,
        |req: Request<Executed, InsertedUser>| -> PipelineResult<RegisterResponse> {
            let user = req.into_inner();
            Ok(RegisterResponse {
                id: user.id,
                inserted: user.inserted_otpks,
            })
        },
    )
}

pub fn auth_challenge_pipeline() -> Pipeline<AuthChallengeBody, AuthChallengeResponse, PgDatabase> {
    Pipeline::new(
        false,
        |req: Request<Executed, InsertedNonce>| -> PipelineResult<AuthChallengeResponse> {
            let auth_nonce = req.into_inner();
            Ok(AuthChallengeResponse {
                nonce: auth_nonce.nonce,
            })
        },
    )
}
