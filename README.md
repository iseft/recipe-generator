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
- PostgreSQL 16+ (or use Docker: `docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=recipe_password -e POSTGRES_USER=recipe_user -e POSTGRES_DB=recipe_generator postgres:16-alpine`)
- OpenAI API key

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