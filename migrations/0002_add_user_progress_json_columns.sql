-- Add JSON columns to user_progress table for storing completed items and lesson stars
ALTER TABLE user_progress
ADD COLUMN completed_questions JSONB NOT NULL DEFAULT '[]',
ADD COLUMN completed_code_practices JSONB NOT NULL DEFAULT '[]',
ADD COLUMN lesson_stars JSONB NOT NULL DEFAULT '[]';
