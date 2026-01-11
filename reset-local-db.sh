#!/bin/bash
# Script to RESET (drop and recreate) local databases
# WARNING: This will delete all data in both databases!

set -e

CONTAINER_NAME="recipe-postgres"
POSTGRES_USER="recipe_user"
POSTGRES_PASSWORD="recipe_password"
POSTGRES_DB="recipe_generator"

echo "⚠️  WARNING: This will DELETE ALL DATA in both databases!"
echo "Main DB:  ${POSTGRES_DB}"
echo "Test DB:  recipe_generator_test"
echo ""
read -p "Are you sure you want to continue? (yes/no): " -r
echo

if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Check if container is running
if ! docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo "Error: PostgreSQL container '${CONTAINER_NAME}' is not running."
    echo "Run ./create-local-db.sh first."
    exit 1
fi

echo "Resetting databases..."

# Function to recreate a database
recreate_db() {
    local DB_NAME=$1
    echo "Recreating database '${DB_NAME}'..."
    
    # Terminate existing connections
    docker exec -i "${CONTAINER_NAME}" psql -U "${POSTGRES_USER}" -d postgres -c "
        SELECT pg_terminate_backend(pid) 
        FROM pg_stat_activity 
        WHERE datname = '${DB_NAME}' AND pid <> pg_backend_pid();
    " > /dev/null 2>&1 || true
    
    # Drop database (ignore error if it doesn't exist)
    docker exec -i "${CONTAINER_NAME}" psql -U "${POSTGRES_USER}" -d postgres -c "DROP DATABASE IF EXISTS ${DB_NAME};" > /dev/null 2>&1 || true
    
    # Create database
    docker exec -i "${CONTAINER_NAME}" psql -U "${POSTGRES_USER}" -d postgres -c "CREATE DATABASE ${DB_NAME};"
    
    echo "✓ Database '${DB_NAME}' reset"
}

# Recreate both databases
recreate_db "${POSTGRES_DB}"
recreate_db "recipe_generator_test"

echo ""
echo "✓ Reset complete! Both databases are fresh and empty."
echo "Main DB:  postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}"
echo "Test DB:  postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/recipe_generator_test"
echo ""
echo "Migrations will run automatically when you start the backend."
