pub mod redaction;
pub mod store;

pub use redaction::{Redacted, SecretString};
pub use store::{
    describe_secret_store, reject_plaintext_secret_file, DisabledSecretStore, MemorySecretStore,
    SecretStore, SecretStoreReport,
};

pub const LAYER: &str = "secrets";
