use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct Location {
    pub line: Option<String>,
    pub column: Option<String>,
}

pub type Data = Vec<HashMap<String, String>>;

#[derive(Debug, Clone, Serialize)]
pub struct ValueError {
    pub property: Option<String>,
    pub location: Option<Location>,
    pub message: String,
    pub data: Option<Data>,
}

///
/// ``` json
/// {
///     "errors": [
///          {
///              "message": "Could not open connection to the database",
///              "locations": [{"line": 2, "column": 4}],
///              "data": {
///                  "internal_error": "Connection refused"
///              }
///          }
///     ]
/// }
/// ```
///
#[derive(Debug, Clone, Serialize)]
pub struct ValueErrors {
    errors: Vec<ValueError>,
}

impl ValueErrors {
    pub fn new(
        property: Option<String>,
        location: Option<Location>,
        message: String,
        data: Option<Data>,
    ) -> Self {
        Self {
            errors: vec![ValueError {
                property,
                location,
                message,
                data,
            }],
        }
    }

    pub fn to_value(self) -> Value {
        serde_json::to_value(self.errors).expect("fatal error")
    }
}

/// 使用するエラー群
#[derive(Debug, Clone)]
pub enum Errors {
    InternalServerError,
}

///エラーを fmt で文字列として表示させる
impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
