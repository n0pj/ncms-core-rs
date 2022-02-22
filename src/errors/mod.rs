pub mod authorization;
pub mod database;
pub mod http;
pub mod validation;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub property: String,
    pub code: String,
    pub message: String,
}

impl Error {
    pub fn new<T: Display>(property: T, code: T, message: T) -> Self {
        Self {
            property: property.to_string(),
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn to_value(&self) -> serde_json::Value {
        serde_json::json!({
            "property": self.property,
            "code": self.code,
            "message": self.message,
        })
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self.to_value()).unwrap()
    }
}
