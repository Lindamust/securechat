use chrono::{DateTime, Utc};

use crate::models::NonceKey;

#[derive(Debug, Clone)]
pub struct NonceType {
    pub nonce: NonceKey,
    pub expires_at: DateTime<Utc>,
}
