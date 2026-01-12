# AI-Powered Recipe Generator

A web application that generates recipes using AI based on ingredients you have on hand.

> 🦀 **Note:** This is my very first time using Rust! The codebase may reflect learning patterns and could benefit from Rust best practices.

## Architecture

This project uses **Domain-Driven Design (DDD)** with **Clean Architecture** principles, organized by feature modules:

```
backend/src/
├── recipes/         # Recipes feature module (bounded context)
│   ├── adapters/    # HTTP handlers, DTOs, routes
│   ├── application/ # Use cases and business logic
│   ├── domain/      # Core business entities and traits
│   └── infrastructure/ # Concrete implementations (repos, LLM client)
└── shared/          # Cross-cutting concerns (auth, config, db)
```

**Dependency flow:** Infrastructure → Adapters → Application → Domain

Each feature module follows Clean Architecture layers, making it easy to add new features (e.g., `users/`, `notifications/`) alongside `recipes/`.

## Tech Stack

- **Backend:** Rust, Axum, SQLx
- **Frontend:** React, TypeScript, Vite, TailwindCSS
- **Database:** PostgreSQL
- **LLM:** OpenAI API (gpt-4o-mini)
- **Containerization:** Docker, Docker Compose

## Quick Start (Docker Compose)

**Fastest way to run the application** - Everything runs in containers, no local setup needed.

```bash
# 1. Clone the repository
git clone https://github.com/iseft/recipe-generator
cd recipe-generator

# 2. Copy environment file and configure required keys
cp .env.example .env
# Edit .env and set:
#   - OPENAI_API_KEY=sk-your-openai-key
#   - CLERK_SECRET_KEY=sk_test_your-clerk-secret-key
#   - VITE_CLERK_PUBLISHABLE_KEY=pk_test_your-clerk-publishable-key

# 3. Start all services (frontend, backend, database)
docker compose up --build

# 4. Access the application
# Frontend: http://localhost:8080
# Backend API: http://localhost:3000
# Swagger UI: http://localhost:3000/swagger-ui
```

**Getting your keys:**
- **OpenAI API Key**: Get from [platform.openai.com/api-keys](https://platform.openai.com/api-keys)
- **Clerk Keys**: Sign up at [clerk.com](https://clerk.com), create an application, then go to **Configure** → **API Keys** in the dashboard

**Docker commands:**
```bash
# Stop services
docker compose down

# Stop and remove all data
docker compose down -v
```

## Local Development

**For active development** - Run services locally with hot reload, better debugging, and faster iteration.

### Prerequisites

- Rust 1.75+ (`rustup update`)
- Node.js 20+ (use `nvm use`)
- PostgreSQL 16+ (or use Docker - see below)
- OpenAI API key
- Clerk account and API keys (for authentication)

### Setup

1. Clone and configure:
```bash
git clone https://github.com/iseft/recipe-generator
cd recipe-generator
cp .env.example .env
# Edit .env: set OPENAI_API_KEY, CLERK_SECRET_KEY, VITE_CLERK_PUBLISHABLE_KEY, and DATABASE_URL
```

2. Install dependencies:
```bash
npm install
cd frontend && npm install && cd ..
```

3. Run development servers:
```bash
npm run dev
```

- Backend: `http://localhost:3000`
- Frontend: `http://localhost:5173`

### Run Separately

```bash
npm run dev:backend   # Backend only
npm run dev:frontend  # Frontend only
```

### Local Database Setup

**Initial setup**:
```bash
./create-local-db.sh
```

This script will:
- Create and start a PostgreSQL container named `recipe-postgres` (if it doesn't exist)
- Create the main database `recipe_generator` (if it doesn't exist)
- Create the test database `recipe_generator_test` (if it doesn't exist)

**Reset databases** (when migrations break or you need a fresh start):
```bash
./reset-local-db.sh
```

This script will:
- **⚠️ DROP and recreate both databases (ALL DATA LOST)**
- Ask for confirmation before proceeding
- Useful when migration versions conflict

The container uses:
- User: `recipe_user`
- Password: `recipe_password`
- Port: `5432`
- Connection: `postgres://recipe_user:recipe_password@localhost:5432/recipe_generator`

### Run Tests

Before running tests, ensure the test database exists and is reset:
```bash
./create-local-db.sh  # Creates both main and test databases
./reset-local-db.sh   # Reset databases to ensure clean state
```

Then run tests:
```bash
cd backend
cargo test
```

### Format & Lint Checks

```bash
# Format check (backend)
cd backend && cargo fmt --check

# Lint check (backend)
cd backend && cargo clippy -- -D warnings

# Lint check (frontend)
cd frontend && npm run lint

# Run all checks from root
npm run format  # Format check
npm run lint    # Lint check
npm run test    # Run all tests
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/api/recipes/generate` | Generate a recipe from ingredients |
| POST | `/api/recipes` | Save a generated recipe |
| GET | `/api/recipes` | List all saved recipes |
| GET | `/api/recipes/:id` | Get a single recipe |
| GET | `/api/recipes/shared` | List recipes shared with the user |
| POST | `/api/recipes/:id/shares` | Share a recipe with a user |
| DELETE | `/api/recipes/:recipe_id/shares/:user_id` | Remove a share |
| GET | `/api/recipes/:id/shares` | List users a recipe is shared with |

### OpenAPI/Swagger Documentation

Interactive API documentation is available at:
- **Swagger UI**: http://localhost:3000/swagger-ui
- **OpenAPI JSON**: http://localhost:3000/api-doc/openapi.json

The Swagger UI provides an interactive interface to explore all endpoints, view request/response schemas, and test API calls directly from the browser.

## Environment Variables

See `.env.example` for a complete example file.

### Required

| Variable | Description |
|----------|-------------|
| `OPENAI_API_KEY` | OpenAI API key |
| `CLERK_SECRET_KEY` | Clerk secret key (backend) |
| `VITE_CLERK_PUBLISHABLE_KEY` | Clerk publishable key (frontend) |

### Optional

| Variable | Description | Default |
|----------|-------------|---------|
| `POSTGRES_USER` | PostgreSQL user | `recipe_user` |
| `POSTGRES_PASSWORD` | PostgreSQL password | `recipe_password` |
| `POSTGRES_DB` | PostgreSQL database name | `recipe_generator` |
| `DATABASE_URL` | Full database connection string | Constructed from `POSTGRES_*` vars |
| `PORT` | Backend server port | `3000` |
| `CORS_ORIGIN` | Allowed CORS origin | `http://localhost:5173` |
| `DB_PORT` | Database port (Docker) | `5432` |
| `BACKEND_PORT` | Backend port (Docker) | `3000` |
| `FRONTEND_PORT` | Frontend port (Docker) | `8080` |

## Docker Services

The application runs as three containers:

| Service | Description | Port |
|---------|-------------|------|
| `frontend` | React app served via nginx | 8080 |
| `backend` | Rust API server | 3000 |
| `db` | PostgreSQL database | 5432 |

All services include health checks. The frontend nginx proxies `/api` requests to the backend.

## Database Migrations

Migrations run automatically on backend startup. Migration files are in `backend/migrations/`.

## Authentication Setup (Clerk)

This application uses [Clerk](https://clerk.com) for authentication.

### Getting Your Clerk Keys

1. Sign up at [clerk.com](https://clerk.com) and create an application
2. In the Clerk Dashboard (https://dashboard.clerk.com/), go to **Configure** -> **API Keys**
3. Copy the **Secret Key** → set as `CLERK_SECRET_KEY` in your `.env` file
4. Copy the **Publishable Key** → set as `VITE_CLERK_PUBLISHABLE_KEY` in your `.env` file

### How It Works

The app uses Clerk's React components which handle authentication automatically. The backend verifies JWT tokens from the frontend using Clerk's JWT verification library.

## AI Tools Usage

This project was developed with the assistance of AI tools (specifically Cursor's AI coding assistant) to accelerate development and ensure best practices. AI was used for:

- **Code generation**: Initial scaffolding of components, handlers, and database models
- **Refactoring assistance**: Help with architectural decisions (DDD structure, Clean Architecture patterns)
- **Error resolution**: Debugging compilation errors and type mismatches
- **Documentation**: Generating README sections and code comments
- **Code review**: Suggestions for improvements and consistency

All AI-generated code was reviewed, tested, and integrated manually. The final codebase reflects deliberate architectural choices and follows Rust/TypeScript best practices.

## Security Considerations

### Authentication & Authorization

- **JWT Verification**: All protected endpoints verify JWT tokens from Clerk
- **Access Control**: 
  - Recipe owners can view, save, and share their recipes
  - Shared users can only view recipes shared with them
  - Unauthenticated users can only generate recipes (cannot save or share)
- **User Isolation**: Recipes are scoped to their owner (`owner_id` field)

### API Security

- **Protected Endpoints**: All endpoints except `/health` and `/api/recipes/generate` require authentication
- **Error Handling**: Detailed error messages are logged server-side but generic messages are returned to clients
- **Input Validation**: All API requests are validated using `validator` crate
- **SQL Injection Prevention**: Using parameterized queries via `sqlx`