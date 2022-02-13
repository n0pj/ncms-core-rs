use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

///
/// パスワードのハッシュ化と確認用
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub iat: i64,
    pub exp: i64,
}

///
/// Example: claims
/// ```rust
/// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// pub struct Claims {
///     pub user_id: String,
///     pub iat: i64,
///     pub exp: i64,
/// }
/// ```
pub fn gen_jwt_token<T: Serialize>(
    claims: T,
    secret: &str,
    algorithm: Option<Algorithm>,
    typ: Option<String>,
) -> String {
    let mut header = Header::new(algorithm.unwrap_or(Algorithm::HS256));
    // let now = Utc::now();
    // let iat = now.clone().timestamp();
    // let exp = (now + chrono::Duration::days(30)).timestamp();
    // let claims = Claims { user_id, iat, exp };

    header.typ = Some(typ.unwrap_or("JWT".to_string()));

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Unable to generate token")
}

pub fn verify_jwt_token<T: DeserializeOwned>(
    token: &str,
    secret: &str,
) -> Result<TokenData<T>, jsonwebtoken::errors::Error> {
    decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_password_failed() {
        let saved_password = "passwords";
        let saved_password = gen_password_hash(&saved_password).unwrap();

        let input_password = "password";
        let result = verify_password(input_password, &saved_password);

        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn verify_password_pass() {
        let saved_password = "password";
        let saved_password = gen_password_hash(&saved_password).unwrap();

        let input_password = "password";
        let result = verify_password(input_password, &saved_password);

        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn gen_jwt_token_pass() {
        let claims = Claims {
            user_id: "test".to_owned(),
            iat: 1,
            exp: 2,
        };
        let secret = "secret";
        let algorithm = Some(Algorithm::HS256);
        let typ = Some("JWT".to_string());

        let token = gen_jwt_token(claims, secret, algorithm, typ);

        assert!(token.len() > 0);
    }

    #[test]
    fn verify_jwt_token_pass() {
        let claims = Claims {
            user_id: "test".to_owned(),
            iat: 1,
            exp: 9999999999,
        };
        let secret = "secret";
        let algorithm = Some(Algorithm::HS256);
        let typ = Some("JWT".to_string());
        let token = gen_jwt_token(claims, secret, algorithm, typ);
        let result = verify_jwt_token::<Claims>(&token, secret);

        assert!(result.is_ok());
    }

    // wrong secret
    #[test]
    fn verify_jwt_token_failed() {
        let claims = Claims {
            user_id: "test".to_owned(),
            iat: 1,
            exp: 9999999999,
        };
        let secret = "secret";
        let algorithm = Some(Algorithm::HS256);
        let typ = Some("JWT".to_string());
        let token = gen_jwt_token(claims, secret, algorithm, typ);

        // wrong secret
        let result = verify_jwt_token::<Claims>(&token, "secret2");

        assert!(result.is_err());
    }
}
