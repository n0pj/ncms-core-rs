use juniper::ScalarValue;
use serde::{de, de::StdError, Deserialize, Serialize};
use std::fmt;

/// GraphQL Custom ScalarValue
/// 実装合ってるかわからん
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomScalarValue {
    Int(i32),
    BigInt(i64),
    String(String),
}

impl std::convert::From<i32> for CustomScalarValue {
    fn from(i: i32) -> Self {
        CustomScalarValue::Int(i)
    }
}

impl std::convert::From<bool> for CustomScalarValue {
    fn from(b: bool) -> Self {
        CustomScalarValue::String(b.to_string())
    }
}

impl std::convert::From<String> for CustomScalarValue {
    fn from(s: String) -> Self {
        CustomScalarValue::String(s)
    }
}

impl std::convert::From<i64> for CustomScalarValue {
    fn from(i: i64) -> Self {
        CustomScalarValue::BigInt(i)
    }
}

impl std::convert::From<f64> for CustomScalarValue {
    fn from(f: f64) -> Self {
        CustomScalarValue::String(f.to_string())
    }
}

impl fmt::Display for CustomScalarValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomScalarValue::Int(i) => write!(f, "{}", i),
            CustomScalarValue::BigInt(i) => write!(f, "{}", i),
            CustomScalarValue::String(s) => write!(f, "{}", s),
        }
    }
}

impl StdError for CustomScalarValue {}

impl de::Error for CustomScalarValue {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        CustomScalarValue::String(msg.to_string())
    }
}

impl ScalarValue for CustomScalarValue {
    type Visitor = CustomScalarValueVisitor;

    fn as_int(&self) -> Option<i32> {
        match *self {
            Self::Int(ref i) => Some(*i),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<String> {
        match *self {
            Self::String(ref s) => Some(s.clone()),
            _ => None,
        }
    }

    fn into_string(self) -> Option<String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match *self {
            Self::String(ref s) => Some(s.as_str()),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f64> {
        match *self {
            // Self::Int(ref i) => Some(*i as f64),
            // Self::Float(ref f) => Some(*f),
            _ => None,
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        match *self {
            // Self::Boolean(ref b) => Some(*b),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct CustomScalarValueVisitor;

impl<'de> de::Visitor<'de> for CustomScalarValueVisitor {
    type Value = CustomScalarValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a custom scalar value")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CustomScalarValue::BigInt(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CustomScalarValue::BigInt(value as i64))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(de::Error::invalid_type(
            de::Unexpected::Float(value),
            &"a custom scalar value",
        ))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CustomScalarValue::String(value.to_owned()))
    }
}
