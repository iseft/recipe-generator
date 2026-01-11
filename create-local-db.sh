#!/bin/bash
# Script to create local PostgreSQL container and databases for development and testing

set -e

CONTAINER_NAME="recipe-postgres"
POSTGRES_USER="recipe_user"
POSTGRES_PASSWORD="recipe_password"
POSTGRES_DB="recipe_generator"

# Check if container exists
if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    # Container exists, check if it's running
    if ! docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
        echo "Starting existing container '${CONTAINER_NAME}'..."
        docker start "${CONTAINER_NAME}"
    else
        echo "Container '${CONTAINER_NAME}' is already running"
    fi
else
    # Container doesn't exist, create it
    echo "Creating PostgreSQL container '${CONTAINER_NAME}'..."
    docker run -d \
        --name "${CONTAINER_NAME}" \
        -e POSTGRES_USER="${POSTGRES_USER}" \
        -e POSTGRES_PASSWORD="${POSTGRES_PASSWORD}" \
        -e POSTGRES_DB="${POSTGRES_DB}" \
        -p 5432:5432 \
        postgres:16-alpine
fi

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
until docker exec "${CONTAINER_NAME}" pg_isready -U "${POSTGRES_USER}" > /dev/null 2>&1; do
    echo -n "."
    sleep 1
done
echo ""

echo "Creating local databases..."

# Create the main database (ignore error if it already exists)
if docker exec -i "${CONTAINER_NAME}" psql -U "${POSTGRES_USER}" -d postgres -c "CREATE DATABASE ${POSTGRES_DB};" 2>/dev/null; then
    echo "✓ Created database '${POSTGRES_DB}'"
else
    echo "✓ Database '${POSTGRES_DB}' already exists"
fi

# Create the test database (ignore error if it already exists)
if docker exec -i "${CONTAINER_NAME}" psql -U "${POSTGRES_USER}" -d postgres -c "CREATE DATABASE recipe_generator_test;" 2>/dev/null; then
    echo "✓ Created database 'recipe_generator_test'"
else
    echo "✓ Database 'recipe_generator_test' already exists"
fi

echo ""
echo "✓ Setup complete!"
echo "Database connection: postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}"
