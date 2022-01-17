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
///              "property": "key",
///              "location": {"line": 2, "column": 4},
///              "message": "Could not open connection to the database",
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
    pub errors: Vec<ValueError>,
}

/// implementation for ValueError default
impl Default for ValueError {
    fn default() -> Self {
        Self {
            property: None,
            location: None,
            message: "".to_owned(),
            data: None,
        }
    }
}

impl ValueErrors {
    pub fn new(errors: Vec<ValueError>) -> Self {
        Self { errors: errors }
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
