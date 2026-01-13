-- SOLVER-Ralph PostgreSQL Initialization Script
-- This script creates the required databases for self-host deployment.
-- Applied automatically when the postgres container first starts.

-- Create the Zitadel database (required by Zitadel)
CREATE DATABASE zitadel;

-- Grant permissions
GRANT ALL PRIVILEGES ON DATABASE solver_ralph TO postgres;
GRANT ALL PRIVILEGES ON DATABASE zitadel TO postgres;

-- Create extensions in the main database
\c solver_ralph;

-- Required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Note: The actual schema migrations are applied by the application
-- via sqlx migrations. This init script only creates the database
-- and required extensions.
