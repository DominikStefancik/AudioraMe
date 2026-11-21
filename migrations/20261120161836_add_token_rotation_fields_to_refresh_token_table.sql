-- Migration 0004: Add token rotation fields to refresh_token table

ALTER TABLE refresh_token
ADD COLUMN expires_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (NOW() + INTERVAL '7 days'),
ADD COLUMN is_used BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN used_at TIMESTAMP WITH TIME ZONE;

-- Add index for checking expired tokens (for cleanup jobs)
CREATE INDEX idx_refresh_token_expires_at ON refresh_token(expires_at);

-- Add index for checking if token was used (for reuse detection)
CREATE INDEX idx_refresh_token_is_used ON refresh_token(is_used);

-- Update existing tokens to have expiration (7 days from now)
UPDATE refresh_token
SET expires_at = NOW() + INTERVAL '7 days'
WHERE expires_at IS NULL;
