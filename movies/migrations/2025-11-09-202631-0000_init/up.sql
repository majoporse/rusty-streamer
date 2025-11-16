-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- ======================================
-- Trigger function to update updated_at
-- ======================================
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS trigger AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ======================================
-- 1. movies
-- ======================================
CREATE TABLE movies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    description TEXT,
    release_date DATE,
    duration_minutes INT,
    mpaa_rating TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Trigger for movies table
CREATE TRIGGER trg_movies_updated_at
BEFORE UPDATE ON movies
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

-- ======================================
-- 2. people
-- ======================================
CREATE TABLE people (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    birth_date DATE,
    bio TEXT,
    role TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ======================================
-- 3. movie_crew
-- ======================================
CREATE TABLE movie_crew (
    movie_id UUID NOT NULL REFERENCES movies(id) ON UPDATE CASCADE ON DELETE CASCADE,
    person_id UUID NOT NULL REFERENCES people(id) ON UPDATE CASCADE ON DELETE CASCADE,
    role TEXT NOT NULL,
    character_name TEXT,
    billing_order INT,
    PRIMARY KEY (movie_id, person_id, role)
);

-- ======================================
-- 4. reviews
-- ======================================
CREATE TABLE reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    movie_id UUID NOT NULL REFERENCES movies(id) ON UPDATE CASCADE ON DELETE CASCADE,
    user_name TEXT NOT NULL,
    user_id UUID NOT NULL,
    rating SMALLINT NOT NULL,
    title TEXT,
    body TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ======================================
-- 5. genres
-- ======================================
CREATE TABLE genres (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE
);

-- ======================================
-- 6. movie_genres (many-to-many)
-- ======================================
CREATE TABLE movie_genres (
    movie_id UUID NOT NULL REFERENCES movies(id) ON UPDATE CASCADE ON DELETE CASCADE,
    genre_id UUID NOT NULL REFERENCES genres(id) ON UPDATE CASCADE ON DELETE CASCADE,
    PRIMARY KEY (movie_id, genre_id)
);
