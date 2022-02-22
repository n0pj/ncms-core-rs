pub mod authorization;
pub mod database;
pub mod http;
pub mod validation;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error<'a> {
    pub property: &'a str,
    pub code: &'a str,
    pub message: &'a str,
}
