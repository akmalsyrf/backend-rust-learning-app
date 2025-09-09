#!/bin/bash

# Development script for Rust Learning Backend

set -e

echo "🚀 Starting Rust Learning Backend Development Environment"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Start PostgreSQL
echo "📦 Starting PostgreSQL..."
docker-compose up -d postgres

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
until docker-compose exec postgres pg_isready -U postgres; do
    echo "Waiting for PostgreSQL..."
    sleep 2
done

echo "✅ PostgreSQL is ready!"

# Copy environment file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp env.example .env
    echo "✅ .env file created. Please review and update as needed."
fi

# Run migrations
echo "🗄️ Running database migrations..."
cargo run --bin migrate 2>/dev/null || echo "Migration binary not found, skipping..."

# Start the server
echo "🎯 Starting the backend server..."
echo "📊 Admin dashboard will be available at: http://localhost:3000/admin"
echo "🔗 API endpoints will be available at: http://localhost:3000/api"
echo "💡 Press Ctrl+C to stop the server"

cargo run
