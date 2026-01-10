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
- **LLM:** OpenAI API (gpt-4o-mini)
- **Validation:** validator crate with custom Axum extractor

## Prerequisites

- Rust 1.75+ (`rustup update`)
- OpenAI API key

## Setup

1. Clone the repository:
```bash
git clone <repo-url>
cd recipe-generator
```

2. Configure environment:
```bash
cp backend/.env.example backend/.env
# Edit backend/.env and add your OPENAI_API_KEY
```

## Running Locally

### Backend

```bash
cd backend
cargo run
```

Server starts at `http://localhost:3000`

### Run Tests

```bash
cd backend
cargo test
```

## API Usage

### Generate Recipe

```bash
curl -X POST http://localhost:3000/api/recipe \
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