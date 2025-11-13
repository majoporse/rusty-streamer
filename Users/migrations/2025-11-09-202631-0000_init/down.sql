-- =====================================
-- DOWN MIGRATION: Drop all user-related tables
-- =====================================

DROP TABLE IF EXISTS watch_room_messages CASCADE;
DROP TABLE IF EXISTS watch_room_participants CASCADE;
DROP TABLE IF EXISTS watch_rooms CASCADE;
DROP TABLE IF EXISTS watchlist CASCADE;
DROP TABLE IF EXISTS watch_history CASCADE;
DROP TABLE IF EXISTS users CASCADE;
