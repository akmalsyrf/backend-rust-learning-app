use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CodePracticeId(pub Uuid);

impl CodePracticeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| "Invalid UUID format".to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for CodePracticeId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for CodePracticeId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<CodePracticeId> for Uuid {
    fn from(code_practice_id: CodePracticeId) -> Self {
        code_practice_id.0
    }
}
