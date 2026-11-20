-- Migration 0003: Create refresh token table

CREATE TABLE refresh_token (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- "ON DELETE CASCADE" means if we delete aa end_user, PostgreSQL automatically deletes all their tokens.
    end_user_id UUID NOT NULL REFERENCES end_user(id) ON DELETE CASCADE,
    token VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_used_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indices for fast lookups
CREATE INDEX idx_refresh_token_token ON refresh_token(token);
CREATE INDEX idx_refresh_token_end_user_id ON refresh_token(end_user_id);
