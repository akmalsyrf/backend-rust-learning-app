use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{Difficulty, LocalizedText, Points, QuestionId, TopicId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    MultipleChoice {
        choices: LocalizedText,
        correct_index: u32,
    },
    TrueFalse {
        answer: bool,
    },
    FillInBlank {
        acceptable_answers: Vec<String>,
    },
    CodeOutputPrediction {
        code: String,
        expected_stdout: String,
    },
    CodeFix {
        code: String,
        choices: Vec<String>,
        correct_index: u32,
    },
    CodeWriting {
        scaffold: String,
        validators: CodeValidators,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeValidators {
    pub must_include: Option<Vec<String>>,
    pub must_not_include: Option<Vec<String>>,
    pub test_cases: Option<Vec<TestCase>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub expected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: QuestionId,
    pub prompt: LocalizedText,
    pub explanation: Option<LocalizedText>,
    pub topic_id: TopicId,
    pub difficulty: Difficulty,
    pub points: Points,
    pub question_type: QuestionType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Question {
    pub fn new(
        prompt: LocalizedText,
        topic_id: TopicId,
        difficulty: Difficulty,
        points: Points,
        question_type: QuestionType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: QuestionId::new(),
            prompt,
            explanation: None,
            topic_id,
            difficulty,
            points,
            question_type,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_explanation(&mut self, explanation: LocalizedText) {
        self.explanation = Some(explanation);
        self.updated_at = Utc::now();
    }

    pub fn update_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.updated_at = Utc::now();
    }

    pub fn update_points(&mut self, points: Points) {
        self.points = points;
        self.updated_at = Utc::now();
    }
}
