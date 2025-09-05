use argon2:: {
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString, PasswordVerifier},
    Argon2,
};

pub fn hash_password(password: &[u8]) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(password, &salt)? {
        Some(password_hash) => Ok(password_hash.to_string()),
        None => Err(argon2::password_hash::Error::Password),
    }
}

pub fn verify_password(hash: &str, password: &[u8]) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    argon2.verify_password(password, &parsed_hash)
}
