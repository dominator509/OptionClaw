use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Redacted<T> {
    value: T,
}

impl<T> Redacted<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn expose(&self) -> &T {
        &self.value
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> fmt::Debug for Redacted<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

impl<T> fmt::Display for Redacted<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<redacted>")
    }
}

pub type SecretString = Redacted<String>;

impl From<String> for SecretString {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SecretString {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{Redacted, SecretString};

    #[test]
    fn redaction_hides_sensitive_value() {
        let secret = Redacted::new("opclaw-secret".to_string());
        assert_eq!(format!("{secret}"), "<redacted>");
        assert!(!format!("{secret:?}").contains("opclaw-secret"));
    }

    #[test]
    fn secret_string_can_be_constructed_from_str() {
        let secret: SecretString = "top-secret".into();
        assert_eq!(secret.expose(), "top-secret");
        assert_eq!(format!("{secret}"), "<redacted>");
    }
}
