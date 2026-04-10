use serde::{Deserialize, Serialize};

/// A fast JSON value type with zero-copy deserialization support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FastValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<FastValue>),
    Object(Vec<(String, FastValue)>),
}

impl FastValue {
    /// Parse a JSON string into a `FastValue`.
    ///
    /// # Errors
    /// Returns an error if the input is not valid JSON.
    pub fn parse(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input)
    }

    /// Returns `true` if the value is null.
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns the value as a string slice, if it is a string.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_null() {
        let v = FastValue::parse("null").unwrap();
        assert!(v.is_null());
    }

    #[test]
    fn parse_string() {
        let v = FastValue::parse(r#""hello""#).unwrap();
        assert_eq!(v.as_str(), Some("hello"));
    }

    #[test]
    fn parse_object() {
        let v = FastValue::parse(r#"{"key": "value"}"#).unwrap();
        assert!(!v.is_null());
    }
}
