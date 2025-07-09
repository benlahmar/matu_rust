-- Add migration script here

CREATE TABLE posts (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,

    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_posts_user_id ON posts(user_id);