use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::FromRequestParts,
    http::{HeaderMap, request::Parts},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use pipeline_core::{error::PipelineError, request::Request, stages::Authenticated};

use crate::error::HttpError;

/// JWT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    /// Expiry (Unix timestamp).
    pub exp: u64,
}

use jsonwebtoken::errors::Error as JwtError;


pub fn mint_jwt(sub: Uuid) -> Result<String, JwtError> {
    let curr_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let exp = curr_time + 3600;

    let secret = "SECRET".as_bytes();
    let encoding_key = EncodingKey::from_secret(secret);

    let claims = Claims {
        sub,
        exp,
    };

    encode(&Header::default(), &claims, &encoding_key)
}

/// What we know about the caller *after* the auth stage.
#[derive(Debug, Clone)]
pub enum Identity {
    Authenticated(Claims),
    Anonymous,
}

impl Identity {
    pub fn require_authenticated(&self) -> Result<&Claims, HttpError> {
        match self {
            Identity::Authenticated(c) => Ok(c),
            Identity::Anonymous => Err(PipelineError::MissingToken.into()),
        }
    }

    pub fn user_id(&self) -> Result<Uuid, HttpError> {
        self.require_authenticated().map(|c| c.sub)
    }
}

/// Axum extractor that resolves to an `Identity`.
///
/// Place `AuthIdentity` before the JSON body extractor in handler signatures.
/// For public endpoints, use `MaybeAuth` (which always yields `Anonymous`).
pub struct AuthIdentity(pub Identity);

/// JWT secret
pub struct JwtSecret(pub String);

impl<S> FromRequestParts<S> for AuthIdentity
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let identity = extract_identity(&parts.headers)?;
        Ok(AuthIdentity(identity))
    }
}

fn extract_identity(headers: &HeaderMap) -> Result<Identity, HttpError> {
    let Some(header_value) = headers.get("Authorization") else {
        // No header --> anonymous; callers that need auth will call
        // `identity.require_authenticated()` and get an error there.
        return Ok(Identity::Anonymous);
    };

    let raw = header_value
        .to_str()
        .map_err(|_| PipelineError::MissingToken)?;

    let token = raw
        .strip_prefix("Bearer ")
        .ok_or(PipelineError::MissingToken)?;

    // TODO: replace this temp placeholder
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret".to_owned());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| PipelineError::InvalidToken)?;

    Ok(Identity::Authenticated(token_data.claims))
}

/// Advance a `Raw` request to `Authenticated`, attaching the resolved identity.
///
/// `require_auth`: if `true`, anonymous callers are rejected immediately.
pub fn into_authenticated<T>(
    payload: T,
    identity: Identity,
    require_auth: bool,
) -> Result<Request<Authenticated, (Identity, T)>, HttpError> {
    if require_auth {
        identity.require_authenticated()?;
    }
    Ok(Request::wrap((identity, payload)))
}
