# AI-Powered Recipe Generator

A web application that generates recipes using AI based on ingredients you have on hand.

## Architecture

This project uses **Clean Architecture** with four layers:

```
backend/src/
├── domain/          # Core business entities and traits (innermost)
├── application/     # Use cases and business logic
├── adapters/        # HTTP handlers, DTOs, routes
└── infrastructure/  # External services (OpenAI), config
```

**Dependency flow:** Infrastructure → Adapters → Application → Domain

## Tech Stack

- **Backend:** Rust, Axum
- **Frontend:** React, TypeScript, Vite, TailwindCSS
- **LLM:** OpenAI API (gpt-4o-mini)
- **Validation:** Zod (frontend), validator crate (backend)

## Prerequisites

- Rust 1.75+ (`rustup update`)
- Node.js 20+ (use `nvm use` to switch to the version specified in `.nvmrc`)
- OpenAI API key

## Setup

1. Clone the repository:
```bash
git clone <repo-url>
cd recipe-generator
```

2. Switch to the correct Node.js version (if using nvm):
```bash
nvm use
```

3. Install dependencies:
```bash
npm install
cd frontend && npm install && cd ..
```

3. Configure environment:
```bash
cp .env.example .env
# Edit .env and add your OPENAI_API_KEY
```

## Running Locally

### Development (both frontend and backend)

```bash
npm run dev
```

- Backend: `http://localhost:3000`
- Frontend: `http://localhost:5173`

### Production Build

Build and start in one command:
```bash
npm run build:start
```

Or build and start separately:
```bash
npm run build
npm start
```

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

## API Usage

### Generate Recipe

```bash
curl -X POST http://localhost:3000/api/recipes/generate \
  -H "Content-Type: application/json" \
  -d '{
    "ingredients": ["chicken", "rice", "garlic"],
    "dietaryRestrictions": ["gluten-free"]
  }'
```

**Response:**
```json
{
  "title": "Garlic Chicken and Rice",
  "ingredients": ["2 cups chicken breast", "1 cup rice", "4 cloves garlic"],
  "instructions": ["Step 1...", "Step 2..."],
  "prepTimeMinutes": 10,
  "cookTimeMinutes": 20,
  "servings": 4
}
```

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `PORT` | Server port (default: 3000) | No |
| `OPENAI_API_KEY` | OpenAI API key | Yes |
| `CORS_ORIGIN` | Allowed CORS origin (default: http://localhost:5173) | No |