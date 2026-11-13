-- Migration: Create end_user table

-- This enables PostgreSQL’s UUID generation functions, like "uuid_generate_v4".
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE end_user (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    bio TEXT,
    image VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_end_user_email ON end_user(email);
CREATE INDEX idx_end_user_username ON end_user(username);
CREATE INDEX idx_end_user_created_at ON end_user(created_at);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    -- The NEW variable represents the row being updated, and we modify it before the update completes.
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_end_user_updated_at
    -- This trigger tells PostgreSQL:
    -- “Before any UPDATE happens on the end_user table, run our timestamp function for each affected row.”
    BEFORE UPDATE ON end_user
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
