use bcrypt::BcryptError;

/// Minimum bcrypt cost factor for password hashing.
#[allow(dead_code)]
pub const BCRYPT_COST: u32 = 12;

/// Hash a plaintext password using bcrypt with cost factor >= 12.
#[allow(dead_code)]
pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    bcrypt::hash(password, BCRYPT_COST)
}
