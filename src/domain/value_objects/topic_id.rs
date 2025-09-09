use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TopicId(pub Uuid);

impl TopicId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| "Invalid UUID format".to_string())
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        Self::from_str(&s)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for TopicId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for TopicId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<TopicId> for Uuid {
    fn from(topic_id: TopicId) -> Self {
        topic_id.0
    }
}
