use pipeline::error::PipelineResult;

// Password hashing (TODO)
pub fn hash_password(plain: &str) -> PipelineResult<String> {
    // Replace with argon2::hash_encoded or bcrypt::hash later
    Ok(format!("hashed:{plain}"))
}
