#!/bin/bash

# Development setup script for Rust Learning Backend

set -e

echo "🚀 Setting up Rust Learning Backend development environment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Not in a Rust project directory. Please run this from the backend directory."
    exit 1
fi

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    print_error "Rust is not installed. Please install Rust first: https://rustup.rs/"
    exit 1
fi

print_status "Rust is installed: $(rustc --version)"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    print_error "Cargo is not installed or not in PATH"
    exit 1
fi

print_status "Cargo is available: $(cargo --version)"

# Install Rust components
print_info "Installing Rust components..."
rustup component add rustfmt clippy
print_status "Rust components installed"

# Install useful cargo tools
print_info "Installing useful cargo tools..."
cargo install cargo-watch cargo-audit cargo-tarpaulin cargo-outdated
print_status "Cargo tools installed"

# Install pre-commit if not already installed
if ! command -v pre-commit &> /dev/null; then
    print_info "Installing pre-commit..."
    if command -v pip3 &> /dev/null; then
        pip3 install pre-commit
    elif command -v pip &> /dev/null; then
        pip install pre-commit
    else
        print_warning "pip not found. Please install pre-commit manually: https://pre-commit.com/"
    fi
else
    print_status "Pre-commit is already installed"
fi

# Install pre-commit hooks
if command -v pre-commit &> /dev/null; then
    print_info "Installing pre-commit hooks..."
    pre-commit install
    print_status "Pre-commit hooks installed"
fi

# Set up git hooks
print_info "Setting up git hooks..."
if [ -d ".git" ]; then
    cp scripts/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    print_status "Git hooks configured"
else
    print_warning "Not a git repository. Initialize git first: git init"
fi

# Create .env file if it doesn't exist
if [ ! -f ".env" ]; then
    print_info "Creating .env file from template..."
    cp env.example .env
    print_status ".env file created"
    print_warning "Please review and update .env file with your configuration"
else
    print_status ".env file already exists"
fi

# Build the project
print_info "Building the project..."
cargo build
print_status "Project built successfully"

# Run tests
print_info "Running tests..."
cargo test
print_status "All tests passed"

# Run clippy
print_info "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings
print_status "Clippy checks passed"

# Run rustfmt
print_info "Checking code formatting..."
cargo fmt --all -- --check
print_status "Code formatting is correct"

# Check if Docker is available
if command -v docker &> /dev/null; then
    print_status "Docker is available: $(docker --version)"
    print_info "You can start the development environment with: make dev"
else
    print_warning "Docker is not installed. Install Docker to use the development environment."
fi

print_status "Development environment setup complete! 🎉"
echo ""
print_info "Available commands:"
echo "  make help          - Show all available commands"
echo "  make dev           - Start development environment"
echo "  make test          - Run tests"
echo "  make lint          - Run linter"
echo "  make format        - Format code"
echo "  make check         - Run all quality checks"
echo "  make pre-commit    - Run pre-commit hooks"
echo ""
print_info "Happy coding! 🦀"
