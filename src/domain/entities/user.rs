use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{Email, Password, Points, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password: Password,
    pub display_name: String,
    pub total_xp: Points,
    pub current_streak_days: u32,
    pub highest_streak_days: u32,
    pub last_active_date: chrono::NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: Email, password: Password, display_name: String) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            password,
            display_name,
            total_xp: Points::new(0),
            current_streak_days: 0,
            highest_streak_days: 0,
            last_active_date: now.date_naive(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_xp(&mut self, points: Points) {
        self.total_xp = self.total_xp.add(points);
        self.updated_at = Utc::now();
    }

    pub fn update_streak(&mut self, days: u32) {
        self.current_streak_days = days;
        if days > self.highest_streak_days {
            self.highest_streak_days = days;
        }
        self.updated_at = Utc::now();
    }

    pub fn update_last_active(&mut self) {
        self.last_active_date = Utc::now().date_naive();
        self.updated_at = Utc::now();
    }

    pub fn verify_password(&self, plain_password: &str) -> Result<bool, String> {
        self.password.verify(plain_password)
    }
}
