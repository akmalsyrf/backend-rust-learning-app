use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QuestionId(pub Uuid);

impl QuestionId {
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

impl Default for QuestionId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for QuestionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<QuestionId> for Uuid {
    fn from(question_id: QuestionId) -> Self {
        question_id.0
    }
}
