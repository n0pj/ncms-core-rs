use regex::Regex;
use std::io::{Error, ErrorKind};

pub type Authorization = Option<String>;

pub trait AuthorizationMethods {
    /// Remove "Bearer " from the authorization header;
    fn get_token(&self) -> Result<String, Error>;
}

impl AuthorizationMethods for Authorization {
    fn get_token(&self) -> Result<String, Error> {
        // Judge whether the authorization header is empty.
        let token = match self {
            Some(token) => token.to_string(),
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "No authorization token provided",
                ))
            }
        };

        // new Regex object
        let re = match Regex::new(r"^Bearer (.*)$") {
            Ok(re) => re,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Regex error: {}", e))),
        };

        // match the token
        let caps = match re.captures(&token) {
            Some(caps) => caps,
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Authorization token is invalid",
                ))
            }
        };

        Ok(caps[1].to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub authorization: Authorization,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token() {
        let header = Header {
            authorization: Some("Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c".to_string()),
        };

        let token = header.authorization.get_token().unwrap();
        assert_eq!(token, "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c");
    }
}
