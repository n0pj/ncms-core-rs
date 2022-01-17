use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize)]
pub struct ValueError {
    pub message: String,
    pub data: Vec<HashMap<String, String>>,
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
#[derive(Serialize)]
pub struct ValueErrors {
    errors: Vec<ValueError>,
}

impl ValueErrors {
    pub fn new(message: &str) -> Self {
        Self {
            errors: vec![ValueError {
                message: message.to_owned(),
                data: vec![],
            }],
        }
    }

    pub fn to_value(self) -> Value {
        serde_json::to_value(self.errors).expect("fatal error")
    }
}

/// 使用するエラー群
#[derive(Debug)]
pub enum Errors {
    InternalServerError,
}

///エラーを fmt で文字列として表示させる
impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
