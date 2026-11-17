-- Migration 0002: Create password reset token table

CREATE TABLE password_reset_token (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- "ON DELETE CASCADE" means if we delete an end_user, PostgreSQL automatically deletes all their tokens.
    end_user_id UUID NOT NULL REFERENCES end_user(id) ON DELETE CASCADE,
    token VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Indices for fast lookups
CREATE INDEX idx_password_reset_token_token ON password_reset_token(token);
CREATE INDEX idx_password_reset_token_end_user_id ON password_reset_token(end_user_id);
CREATE INDEX idx_password_reset_token_expires_at ON password_reset_token(expires_at);
