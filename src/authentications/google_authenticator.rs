// ReExport GoogleAuthenticator
pub use google_authenticator::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_code() {
        let auth = GoogleAuthenticator::new();
        let secret = auth.create_secret(32);
        let code = auth.get_code(&secret, 0).unwrap();

        assert!(auth.verify_code(&secret, &code, 1, 0))
    }
}
