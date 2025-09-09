use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn new(plain_password: &str) -> Result<Self, String> {
        // Length validation
        if plain_password.len() < 12 {
            return Err("Password must be at least 12 characters long".to_string());
        }
        if plain_password.len() > 128 {
            return Err("Password must be at most 128 characters long".to_string());
        }

        // Character requirements
        let has_uppercase = plain_password.chars().any(|c| c.is_uppercase());
        let has_lowercase = plain_password.chars().any(|c| c.is_lowercase());
        let has_digit = plain_password.chars().any(|c| c.is_ascii_digit());
        let has_special = plain_password
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

        if !has_uppercase {
            return Err("Password must contain at least one uppercase letter".to_string());
        }
        if !has_lowercase {
            return Err("Password must contain at least one lowercase letter".to_string());
        }
        if !has_digit {
            return Err("Password must contain at least one digit".to_string());
        }
        if !has_special {
            return Err(
                "Password must contain at least one special character (!@#$%^&*()_+-=[]{}|;:,.<>?)"
                    .to_string(),
            );
        }

        // Check for common passwords
        let common_passwords = [
            "password",
            "123456",
            "123456789",
            "qwerty",
            "abc123",
            "password123",
            "admin",
            "letmein",
            "welcome",
            "monkey",
            "1234567890",
            "password1",
            "qwerty123",
            "dragon",
            "master",
            "hello",
            "freedom",
            "whatever",
            "qazwsx",
            "trustno1",
            "jordan",
            "jennifer",
            "zxcvbnm",
            "asdfgh",
            "hunter",
            "buster",
            "soccer",
            "harley",
            "batman",
            "andrew",
            "tigger",
            "sunshine",
            "iloveyou",
            "2000",
            "charlie",
            "robert",
            "thomas",
            "hockey",
            "ranger",
            "daniel",
            "starwars",
            "klaster",
            "112233",
            "george",
            "computer",
            "michelle",
            "jessica",
            "pepper",
            "1234",
            "zoidberg",
            "trustno1",
            "dragon",
            "master",
            "hello",
            "freedom",
            "whatever",
            "qazwsx",
            "trustno1",
            "jordan",
            "jennifer",
            "zxcvbnm",
            "asdfgh",
            "hunter",
            "buster",
            "soccer",
            "harley",
            "batman",
            "andrew",
            "tigger",
            "sunshine",
            "iloveyou",
            "2000",
            "charlie",
            "robert",
            "thomas",
            "hockey",
            "ranger",
            "daniel",
            "starwars",
            "klaster",
            "112233",
            "george",
            "computer",
            "michelle",
            "jessica",
            "pepper",
            "1234",
            "zoidberg",
        ];

        let password_lower = plain_password.to_lowercase();
        for common in &common_passwords {
            if password_lower.contains(common) {
                return Err("Password contains common patterns and is not secure".to_string());
            }
        }

        // Check for repeated characters
        let mut prev_char = None;
        let mut repeat_count = 0;
        for c in plain_password.chars() {
            if Some(c) == prev_char {
                repeat_count += 1;
                if repeat_count >= 3 {
                    return Err(
                        "Password cannot have more than 2 consecutive identical characters"
                            .to_string(),
                    );
                }
            } else {
                repeat_count = 1;
            }
            prev_char = Some(c);
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(plain_password.as_bytes(), &salt)
            .map_err(|e| format!("Failed to hash password: {}", e))?;

        Ok(Self(password_hash.to_string()))
    }

    pub fn from_hash(hash: String) -> Self {
        Self(hash)
    }

    pub fn verify(&self, plain_password: &str) -> Result<bool, String> {
        let parsed_hash =
            PasswordHash::new(&self.0).map_err(|e| format!("Invalid password hash: {}", e))?;

        let argon2 = Argon2::default();
        match argon2.verify_password(plain_password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
