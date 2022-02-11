pub mod authentications;
pub mod db;
pub mod errors;
pub mod http;
pub mod models;

// ReExports
pub use authentications::common::*;
pub use authentications::google_authenticator::*;
pub use models::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
