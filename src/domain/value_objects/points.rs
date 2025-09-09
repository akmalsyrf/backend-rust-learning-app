use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Points(pub u32);

impl Points {
    pub fn new(points: u32) -> Self {
        Self(points)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn add(&self, other: Points) -> Self {
        Self(self.0 + other.0)
    }

    pub fn subtract(&self, other: Points) -> Option<Self> {
        if self.0 >= other.0 {
            Some(Self(self.0 - other.0))
        } else {
            None
        }
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Points {
    fn from(points: u32) -> Self {
        Self::new(points)
    }
}

impl From<Points> for u32 {
    fn from(points: Points) -> Self {
        points.0
    }
}
