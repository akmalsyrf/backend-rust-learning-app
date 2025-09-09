# Rust Learning App Backend

A backend service for the Rust Learning App built with Rust, Axum, Askama, and PostgreSQL using Domain-Driven Design (DDD) principles.

## Tech Stack

- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL with SQLx
- **Templates**: Askama (for admin dashboard)
- **Authentication**: JWT tokens with Argon2 password hashing
- **Architecture**: Domain-Driven Design (DDD)

## Features

- User authentication and authorization
- Topics, lessons, questions, and code practices management
- User progress tracking
- Leaderboard system
- Admin dashboard (Askama templates)
- RESTful API

## Project Structure

```
src/
├── domain/                 # Domain layer (DDD)
│   ├── entities/          # Domain entities
│   ├── value_objects/     # Value objects
│   ├── repositories/      # Repository interfaces
│   └── services/          # Domain services
├── application/           # Application layer
│   ├── use_cases/        # Use cases
│   ├── dtos/             # Data transfer objects
│   └── interfaces/       # Application interfaces
├── infrastructure/        # Infrastructure layer
│   ├── database/         # Database models and connection
│   ├── repositories/     # Repository implementations
│   └── external_services/ # External service integrations
├── presentation/          # Presentation layer
│   ├── api/              # REST API handlers
│   ├── web/              # Web dashboard handlers
│   └── handlers/         # Shared handlers
└── shared/               # Shared utilities
    ├── config/           # Configuration
    ├── errors/           # Error handling
    └── utils/            # Utility functions
```

## Getting Started

### Prerequisites

- Rust 1.75+
- PostgreSQL 15+
- Docker (optional)

### Development Setup

1. **Clone and navigate to backend directory**:
   ```bash
   cd backend
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Set up environment variables**:
   ```bash
   cp env.example .env
   # Edit .env with your configuration
   ```

4. **Start PostgreSQL** (using Docker):
   ```bash
   docker-compose up postgres -d
   ```

5. **Run migrations**:
   ```bash
   cargo run --bin migrate
   ```

6. **Start the server**:
   ```bash
   cargo run
   ```

### Using Docker Compose

```bash
# Start all services
docker-compose up

# Start in background
docker-compose up -d

# View logs
docker-compose logs -f backend
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login
- `POST /api/auth/refresh` - Refresh token

### Content
- `GET /api/topics` - List all topics
- `GET /api/topics/{id}` - Get topic by ID
- `GET /api/lessons` - List lessons
- `GET /api/lessons/{id}` - Get lesson by ID
- `GET /api/questions` - List questions
- `GET /api/questions/{id}` - Get question by ID
- `GET /api/code-practices` - List code practices
- `GET /api/code-practices/{id}` - Get code practice by ID

### Progress & Leaderboard
- `GET /api/progress` - Get user progress
- `POST /api/progress/questions` - Submit question result
- `POST /api/progress/code-practices` - Submit code practice result
- `GET /api/leaderboard` - Get leaderboard

### Admin Dashboard
- `GET /admin` - Admin dashboard
- `GET /admin/topics` - Manage topics
- `GET /admin/lessons` - Manage lessons
- `GET /admin/questions` - Manage questions
- `GET /admin/users` - Manage users

## Database Schema

The database includes the following main tables:

- `users` - User accounts and authentication
- `topics` - Learning topics with localized content
- `lessons` - Individual lessons within topics
- `questions` - Quiz questions with various types
- `code_practices` - Hands-on coding exercises
- `user_progress` - User progress tracking
- `question_results` - Individual question results
- `lesson_results` - Lesson completion results
- `completed_code_practices` - Code practice completions
- `leaderboard` - Weekly and all-time rankings

## Development

### Quick Setup

```bash
# Run the development setup script
./scripts/setup-dev.sh

# Or use make commands
make install
```

### Running Tests

```bash
# Run all tests
make test

# Run tests in watch mode
make test-watch

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Run all quality checks
make check

# Format code
make format

# Run linter
make lint

# Run pre-commit hooks
make pre-commit
```

### Database Migrations

```bash
# Create new migration
make migrate-create NAME=migration_name

# Run migrations
make migrate

# Reset database (WARNING: deletes all data)
make db-reset
```

### Development Environment

```bash
# Start development environment with Docker
make dev

# Or manually
make docker-up
cargo run
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://postgres:password@localhost:5432/rust_learning` |
| `JWT_SECRET` | Secret key for JWT tokens | `your-secret-key-change-in-production` |
| `SERVER_HOST` | Server host | `0.0.0.0` |
| `SERVER_PORT` | Server port | `3000` |
| `CORS_ORIGINS` | Allowed CORS origins | `http://localhost:3000,http://localhost:3001` |
| `RUST_LOG` | Log level | `debug` |

## Contributing

1. Follow DDD principles
2. Write tests for new features
3. Update documentation
4. Follow Rust conventions
5. Use meaningful commit messages

## License

This project is part of the Rust Learning App.
