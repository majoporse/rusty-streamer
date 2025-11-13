-- =====================================
-- UP MIGRATION: Create user and watch-related tables
-- =====================================

-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ======================
-- USERS TABLE
-- ======================
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    profile_picture_url TEXT,
    country VARCHAR(5),
    language_preference VARCHAR(5),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_login_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(20) DEFAULT 'active' CHECK (status IN ('active', 'suspended', 'deleted'))
);

-- ======================
-- WATCH HISTORY TABLE
-- ======================
CREATE TABLE watch_history (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content_id UUID NOT NULL,
    progress_seconds INTEGER DEFAULT 0,
    completed BOOLEAN DEFAULT FALSE,
    last_watched_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (user_id, content_id)
);

-- ======================
-- WATCHLIST TABLE
-- ======================
CREATE TABLE watchlist (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content_id UUID NOT NULL,
    added_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (user_id, content_id)
);

-- ======================
-- WATCH ROOMS TABLE
-- ======================
CREATE TABLE watch_rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    host_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content_id UUID NOT NULL,
    room_name VARCHAR(100),
    is_private BOOLEAN DEFAULT FALSE,
    invite_code VARCHAR(10) UNIQUE,
    current_time_seconds INTEGER DEFAULT 0,
    is_live BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- ======================
-- WATCH ROOM PARTICIPANTS TABLE
-- ======================
CREATE TABLE watch_room_participants (
    id BIGSERIAL PRIMARY KEY,
    room_id UUID NOT NULL REFERENCES watch_rooms(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_active_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_host BOOLEAN DEFAULT FALSE,
    UNIQUE (room_id, user_id)
);

-- ======================
-- WATCH ROOM MESSAGES TABLE
-- ======================
CREATE TABLE watch_room_messages (
    id BIGSERIAL PRIMARY KEY,
    room_id UUID NOT NULL REFERENCES watch_rooms(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    message TEXT NOT NULL,
    sent_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_system_message BOOLEAN DEFAULT FALSE
);

-- Indexes for faster lookups
CREATE INDEX idx_watch_history_user ON watch_history (user_id);
CREATE INDEX idx_watchlist_user ON watchlist (user_id);
CREATE INDEX idx_watch_room_participants_room ON watch_room_participants (room_id);
CREATE INDEX idx_watch_room_messages_room ON watch_room_messages (room_id);
