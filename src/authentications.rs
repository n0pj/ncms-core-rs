use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub struct Password;

///
/// パスワードのハッシュ化と確認用
///
impl Password {
    ///
    /// https://github.com/RustCrypto/password-hashes/tree/master/argon2
    ///
    pub fn gen_password_hash(raw_password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(raw_password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(
        input_password: &str,
        saved_password: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(saved_password)?;

        Ok(Argon2::default()
            .verify_password(input_password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vverify_password_failed() {
        let saved_password = "passwords";
        let saved_password = Password::gen_password_hash(&saved_password).unwrap();

        let input_password = "password";
        let result = Password::verify_password(input_password, &saved_password);

        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn vverify_password_pass() {
        let saved_password = "password";
        let saved_password = Password::gen_password_hash(&saved_password).unwrap();

        let input_password = "password";
        let result = Password::verify_password(input_password, &saved_password);

        assert_eq!(result.unwrap(), true);
    }
}
