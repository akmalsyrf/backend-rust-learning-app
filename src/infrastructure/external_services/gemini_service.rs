use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GeminiService {
    client: Client,
    api_key: String,
    api_url: String,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    top_k: u32,
    top_p: f32,
    max_output_tokens: u32,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

impl GeminiService {
    pub fn new(api_key: String, api_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key,
            api_url,
        }
    }

    pub async fn generate_quiz_questions(
        &self,
        topic: &str,
        difficulty: &str,
        count: u32,
    ) -> Result<Vec<String>> {
        let prompt = format!(
            "Generate {count} quiz questions about {topic} with {difficulty} difficulty level. \
            Each question should be a multiple choice question with 4 options (A, B, C, D) \
            and clearly indicate the correct answer. Format each question as follows:\n\n\
            Q1: [Question text]\n\
            A) [Option A]\n\
            B) [Option B]\n\
            C) [Option C]\n\
            D) [Option D]\n\
            Answer: [Correct option letter]\n\n\
            Continue this format for all questions.",
        );

        let response = self.call_gemini_api(&prompt).await?;

        // Parse the response to extract individual questions
        let questions = self.parse_quiz_questions(&response)?;

        Ok(questions)
    }

    pub async fn validate_code(&self, code: &str, expected_output: &str) -> Result<bool> {
        let prompt = format!(
            "Analyze this Rust code and determine if it would produce the expected output.\n\n\
            Code:\n```rust\n{code}\n```\n\n\
            Expected output: {expected_output}\n\n\
            Please respond with only 'TRUE' if the code produces the expected output, \
            or 'FALSE' if it doesn't. Be precise in your analysis.",
        );

        let response = self.call_gemini_api(&prompt).await?;

        // Parse the response to determine if code is valid
        let is_valid = response.trim().to_uppercase().contains("TRUE");

        Ok(is_valid)
    }

    pub async fn generate_code_explanation(&self, code: &str) -> Result<String> {
        let prompt = format!(
            "Explain this Rust code in detail, including:\n\
            1. What the code does\n\
            2. How it works step by step\n\
            3. Key Rust concepts used\n\
            4. Any potential improvements or best practices\n\n\
            Code:\n```rust\n{code}\n```",
        );

        self.call_gemini_api(&prompt).await
    }

    pub async fn suggest_code_improvements(&self, code: &str) -> Result<String> {
        let prompt = format!(
            "Review this Rust code and suggest improvements for:\n\
            1. Performance optimization\n\
            2. Code readability\n\
            3. Rust best practices\n\
            4. Error handling\n\
            5. Memory safety\n\n\
            Code:\n```rust\n{code}\n```\n\n\
            Provide specific suggestions with examples where possible.",
        );

        self.call_gemini_api(&prompt).await
    }

    async fn call_gemini_api(&self, prompt: &str) -> Result<String> {
        let url = format!(
            "{}/models/gemini-pro:generateContent?key={}",
            self.api_url, self.api_key
        );

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
            generation_config: GenerationConfig {
                temperature: 0.7,
                top_k: 40,
                top_p: 0.95,
                max_output_tokens: 2048,
            },
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Gemini API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;

        if let Some(candidate) = gemini_response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                return Ok(part.text.clone());
            }
        }

        Err(anyhow::anyhow!("No response content from Gemini API"))
    }

    fn parse_quiz_questions(&self, response: &str) -> Result<Vec<String>> {
        let mut questions = Vec::new();
        let lines: Vec<&str> = response.lines().collect();

        let mut current_question = String::new();
        let mut in_question = false;

        for line in lines {
            let line = line.trim();

            if line.starts_with("Q") && line.contains(':') {
                if !current_question.is_empty() {
                    questions.push(current_question.trim().to_string());
                }
                current_question = line.to_string();
                in_question = true;
            } else if in_question
                && (line.starts_with("A)")
                    || line.starts_with("B)")
                    || line.starts_with("C)")
                    || line.starts_with("D)")
                    || line.starts_with("Answer:"))
            {
                current_question.push_str("\n");
                current_question.push_str(line);
            } else if line.starts_with("Answer:") {
                current_question.push_str("\n");
                current_question.push_str(line);
                questions.push(current_question.trim().to_string());
                current_question.clear();
                in_question = false;
            }
        }

        // Add the last question if it exists
        if !current_question.is_empty() {
            questions.push(current_question.trim().to_string());
        }

        Ok(questions)
    }
}
