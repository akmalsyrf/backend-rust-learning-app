// External services module
// This module handles integrations with external services

pub mod email_service;
pub mod gemini_service;

pub use email_service::*;
pub use gemini_service::*;
