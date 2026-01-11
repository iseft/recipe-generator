# AI-Powered Recipe Generator

A web application that generates recipes using AI based on ingredients you have on hand.

## Architecture

This project uses **Clean Architecture** with four layers:

```
backend/src/
├── domain/          # Core business entities and traits (innermost)
├── application/     # Use cases and business logic
├── adapters/        # HTTP handlers, DTOs, routes
└── infrastructure/  # External services (OpenAI), config, database
```

**Dependency flow:** Infrastructure → Adapters → Application → Domain

## Tech Stack

- **Backend:** Rust, Axum, SQLx
- **Frontend:** React, TypeScript, Vite, TailwindCSS
- **Database:** PostgreSQL
- **LLM:** OpenAI API (gpt-4o-mini)
- **Containerization:** Docker, Docker Compose

## Quick Start (Docker)

The fastest way to run the entire application:

```bash
# 1. Copy environment file and add your OpenAI API key
cp .env.example .env
# Edit .env and set OPENAI_API_KEY=sk-your-key

# 2. Start all services
docker compose up --build

# 3. Access the application
# Frontend: http://localhost:8080
# Backend API: http://localhost:3000
```

To stop:
```bash
docker compose down
```

To stop and remove data:
```bash
docker compose down -v
```

## Local Development

### Prerequisites

- Rust 1.75+ (`rustup update`)
- Node.js 20+ (use `nvm use`)
- PostgreSQL 16+ (or use Docker - see below)
- OpenAI API key

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

### Setup

1. Clone and configure:
```bash
git clone <repo-url>
cd recipe-generator
cp .env.example .env
# Edit .env: set OPENAI_API_KEY and DATABASE_URL
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

### Run Tests

Before running tests, ensure the test database exists:
```bash
./create-local-db.sh  # Creates both main and test databases
```

Then run tests:
```bash
cd backend
cargo test
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/api/recipes/generate` | Generate a recipe from ingredients |
| POST | `/api/recipes` | Save a generated recipe |
| GET | `/api/recipes` | List all saved recipes |
| GET | `/api/recipes/:id` | Get a single recipe |

### Generate Recipe

```bash
curl -X POST http://localhost:3000/api/recipes/generate \
  -H "Content-Type: application/json" \
  -d '{
    "ingredients": ["chicken", "rice", "garlic"],
    "dietaryRestrictions": ["gluten-free"]
  }'
```

### Save Recipe

```bash
curl -X POST http://localhost:3000/api/recipes \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Garlic Chicken and Rice",
    "ingredients": ["2 cups chicken breast", "1 cup rice"],
    "instructions": ["Step 1...", "Step 2..."],
    "prepTimeMinutes": 10,
    "cookTimeMinutes": 20,
    "servings": 4
  }'
```

### List Recipes

```bash
curl http://localhost:3000/api/recipes
```

### Get Recipe by ID

```bash
curl http://localhost:3000/api/recipes/<uuid>
```

## Environment Variables

### Required
| Variable | Description | Required |
|----------|-------------|----------|
| `OPENAI_API_KEY` | OpenAI API key | Yes |
| `POSTGRES_USER` | PostgreSQL user | Yes |
| `POSTGRES_PASSWORD` | PostgreSQL password | Yes |
| `POSTGRES_DB` | PostgreSQL database name | Yes |
| `CLERK_SECRET_KEY` | Clerk secret key (backend) | Yes |
| `VITE_CLERK_PUBLISHABLE_KEY` | Clerk publishable key (frontend) | Yes |

### Optional (with defaults)
| Variable | Description | Default |
|----------|-------------|---------|
| `DB_HOST` | Database host | localhost (local) / db (Docker) |
| `DB_PORT` | Database port | 5432 |
| `PORT` | Backend server port | 3000 |
| `CORS_ORIGIN` | Allowed CORS origin | http://localhost:5173 (local) / http://localhost:8080 (Docker) |
| `FRONTEND_PORT` | Frontend port (Docker) | 8080 |
| `BACKEND_PORT` | Backend port (Docker) | 3000 |
| `VITE_API_URL` | Frontend API URL (leave empty for nginx proxy) | "" |

**Note:** `DATABASE_URL` is automatically constructed from `POSTGRES_*` vars. For Docker, set `DB_HOST=db` in docker-compose (already configured).

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

That's it! No additional configuration needed. The app uses Clerk's React components which handle authentication automatically, and the backend verifies JWT tokens from the frontend.

### Example .env File

```bash
# OpenAI
OPENAI_API_KEY=sk-your-openai-key

# Database
POSTGRES_USER=recipe_user
POSTGRES_PASSWORD=recipe_password
POSTGRES_DB=recipe_generator
DB_HOST=localhost
DB_PORT=5432

# Clerk Authentication
CLERK_SECRET_KEY=sk_test_your-clerk-secret-key
VITE_CLERK_PUBLISHABLE_KEY=pk_test_your-clerk-publishable-key

# Server
PORT=3000
CORS_ORIGIN=http://localhost:5173
```

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