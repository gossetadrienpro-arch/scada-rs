use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegisterValue {
    Bool(bool),
    UInt16(u16),
    Float32(f32),
}

#[derive(Debug, Error,)]
pub enum ScadaError {
    #[error("Connexion échouée vers {host} : {reason}")]
    ConnectionFailed { host: String, reason: String },

    #[error("Adresse invalide : {0}")]
    InvalidAddress(u16),

    #[error("Timeout après {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },

    #[error("Trame malformée : {0}")]
    ParseError(String),

    #[error("Function code non supporté : {0}")]
    UnsupportedFunctionCode(u8),
}

pub type ScadaResult<T> = Result<T, ScadaError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub value: Option<RegisterValue>,
    pub address:u16,
}

impl Tag {
    pub fn new(id: u32, name: &str, address:u16) -> Self {
        Self {
            id,
            name: name.to_string(),
            address,
            value: None,
        }
    }

    pub fn is_acquired(&self) -> bool {
        self.value.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_starts_unacquired() {
        let tag = Tag::new(1, "Température", 40001);
        assert!(!tag.is_acquired());
    }

    #[test]
    fn tag_acquired_after_value_set() {
        let mut tag = Tag::new(2, "Pression", 40002);
        tag.value = Some(RegisterValue::Float32(5.45));
        assert!(tag.is_acquired());
    }
}