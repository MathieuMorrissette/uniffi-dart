use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FriendlyGreeter {
    phrase: String,
}

impl FriendlyGreeter {
    pub fn new(phrase: String) -> Self {
        Self { phrase }
    }

    pub fn greet(&self, name: String) -> String {
        format!("{} {name}", self.phrase)
    }
}

impl std::fmt::Display for FriendlyGreeter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FriendlyGreeter({})", self.phrase)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, uniffi::Object)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct ProcFriendlyGreeter {
    phrase: String,
}

#[uniffi::export]
impl ProcFriendlyGreeter {
    #[uniffi::constructor]
    fn new(phrase: String) -> Arc<Self> {
        Arc::new(Self { phrase })
    }

    fn greet(&self, name: String) -> String {
        format!("{} {}", self.phrase.to_uppercase(), name.to_uppercase())
    }
}

impl std::fmt::Display for ProcFriendlyGreeter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProcFriendlyGreeter({})", self.phrase)
    }
}

uniffi::include_scaffolding!("api");
