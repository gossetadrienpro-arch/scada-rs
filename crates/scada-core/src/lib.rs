pub enum RegisterValue {
    Bool(bool),
    UInt16(u16),
    Float32(f32),
}

pub enum ScadaError {
    ConnectionFailed { host: String, reason: String },
    InvalidAddress(u16),
    Timeout { timeout_ms: u64 },
}

pub type ScadaResult<T> = Result<T, ScadaError>;

pub struct Tag {
    pub id: u32,
    pub name: String,
    pub value: Option<RegisterValue>,
}

impl Tag {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
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
        let tag = Tag::new(1, "Température");
        assert!(!tag.is_acquired());
    }

    #[test]
    fn tag_acquired_after_value_set() {
        let mut tag = Tag::new(2, "Pression"); // étape 1 — à toi de remplir
        tag.value = Some(RegisterValue::Float32(5.45)); // étape 2
        assert!(tag.is_acquired()); // étape 3 — sans le ! cette fois, pourquoi ?
    }
}
