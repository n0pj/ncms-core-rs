// ReExport GoogleAuthenticator
pub use google_authenticator::GoogleAuthenticator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_code() {
        let secret = "JBSWY3DPEHPK3PXP";
        let auth = GoogleAuthenticator::new();
        let code = auth.get_code(secret, 0).unwrap();

        assert!(auth.verify_code(secret, code.as_str(), 1, 0))
    }
}
