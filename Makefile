# Makefile for Rust Learning Backend

.PHONY: help install dev test lint format check build run clean docker-up docker-down migrate

# Default target
help: ## Show this help message
	@echo "Rust Learning Backend - Available commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Installation
install: ## Install dependencies and pre-commit hooks
	@echo "Installing Rust dependencies..."
	cargo build
	@echo "Installing pre-commit hooks..."
	pre-commit install
	@echo "Setting up git hooks..."
	git config core.hooksPath .git/hooks

# Development
dev: ## Start development environment
	@echo "Starting development environment..."
	./scripts/dev.sh

# Testing
test: ## Run all tests
	@echo "Running tests..."
	cargo test
	cargo test --doc

test-watch: ## Run tests in watch mode
	@echo "Running tests in watch mode..."
	cargo watch -x test

# Linting and formatting
lint: ## Run clippy linter
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features

lint-fix: ## Run clippy and try to fix issues
	@echo "Running clippy with fixes..."
	cargo clippy --fix --allow-dirty --allow-staged

format: ## Format code with rustfmt
	@echo "Formatting code..."
	cargo fmt --all

format-check: ## Check if code is formatted correctly
	@echo "Checking code formatting..."
	cargo fmt --all -- --check

# Code quality
check: format-check lint ## Run all code quality checks
	@echo "Running cargo check..."
	cargo check --all-targets --all-features

# Building
build: ## Build the project
	@echo "Building project..."
	cargo build

build-release: ## Build release version
	@echo "Building release version..."
	cargo build --release

# Running
run: ## Run the application
	@echo "Running application..."
	cargo run

run-release: ## Run release version
	@echo "Running release version..."
	cargo run --release

# Database
migrate: ## Run database migrations
	@echo "Running database migrations..."
	sqlx migrate run

migrate-create: ## Create a new migration (usage: make migrate-create NAME=migration_name)
	@echo "Creating migration: $(NAME)"
	sqlx migrate add $(NAME)

# Docker
docker-up: ## Start Docker services
	@echo "Starting Docker services..."
	docker-compose up -d

docker-down: ## Stop Docker services
	@echo "Stopping Docker services..."
	docker-compose down

docker-build: ## Build Docker image
	@echo "Building Docker image..."
	docker build -t rust-learning-backend .

docker-run: ## Run Docker container
	@echo "Running Docker container..."
	docker run -p 3000:3000 rust-learning-backend

# Pre-commit
pre-commit: ## Run pre-commit hooks on all files
	@echo "Running pre-commit hooks..."
	pre-commit run --all-files

pre-commit-update: ## Update pre-commit hooks
	@echo "Updating pre-commit hooks..."
	pre-commit autoupdate


# Documentation
docs: ## Generate documentation
	@echo "Generating documentation..."
	cargo doc --no-deps --open

docs-serve: ## Serve documentation locally
	@echo "Serving documentation..."
	cargo doc --no-deps
	python3 -m http.server 8000 -d target/doc

# CI/CD
ci: check test ## Run CI pipeline locally
	@echo "Running CI pipeline..."

# Database management
db-reset: ## Reset database (WARNING: This will delete all data)
	@echo "Resetting database..."
	docker-compose down -v
	docker-compose up -d postgres
	sleep 5
	make migrate

# Development utilities
watch: ## Watch for changes and rebuild
	@echo "Watching for changes..."
	cargo watch -x run

bench: ## Run benchmarks
	@echo "Running benchmarks..."
	cargo bench

# Git hooks
git-hooks: ## Install git hooks
	@echo "Installing git hooks..."
	cp scripts/pre-commit .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
