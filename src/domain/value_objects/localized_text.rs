use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedText {
    pub en: String,
    pub id: String,
}

impl LocalizedText {
    pub fn new(en: String, id: String) -> Self {
        Self { en, id }
    }

    pub fn from_english(text: String) -> Self {
        Self {
            en: text.clone(),
            id: text,
        }
    }

    pub fn get(&self, language: &str) -> &str {
        match language {
            "id" => &self.id,
            _ => &self.en,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.en.is_empty() && self.id.is_empty()
    }
}

impl From<String> for LocalizedText {
    fn from(text: String) -> Self {
        Self::from_english(text)
    }
}
