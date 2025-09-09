use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Beginner => "beginner",
            Difficulty::Intermediate => "intermediate",
            Difficulty::Advanced => "advanced",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "beginner" => Ok(Difficulty::Beginner),
            "intermediate" => Ok(Difficulty::Intermediate),
            "advanced" => Ok(Difficulty::Advanced),
            _ => Err(format!("Invalid difficulty: {}", s)),
        }
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        Self::from_str(&s)
    }

    pub fn points_multiplier(&self) -> f32 {
        match self {
            Difficulty::Beginner => 1.0,
            Difficulty::Intermediate => 1.5,
            Difficulty::Advanced => 2.0,
        }
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Difficulty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
