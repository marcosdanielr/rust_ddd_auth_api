use bcrypt::{DEFAULT_COST, hash, verify};

pub struct PasswordHasher;

impl PasswordHasher {
    pub fn hash_password(password: &str) -> Result<String, String> {
        hash(password, DEFAULT_COST).map_err(|e| e.to_string())
    }

    pub fn verify_password(password: &str, hashead: &str) -> Result<bool, String> {
        verify(password, hashead).map_err(|e| e.to_string())
    }
}
