-- Your SQL goes here


CREATE TABLE "actors"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"first_name" TEXT NOT NULL,
	"last_name" TEXT NOT NULL,
	"birth_date" DATE,
	"bio" TEXT,
	"created_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "movies"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"title" TEXT NOT NULL,
	"slug" TEXT NOT NULL,
	"description" TEXT,
	"release_date" DATE,
	"duration_minutes" INT4,
	"mpaa_rating" TEXT,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "reviews"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"movie_id" INT4 NOT NULL,
	"user_name" TEXT NOT NULL,
	"rating" INT2 NOT NULL,
	"title" TEXT,
	"body" TEXT,
	"created_at" TIMESTAMPTZ NOT NULL,
	FOREIGN KEY ("movie_id") REFERENCES "movies"("id")
);
CREATE TABLE "movie_actors"(
	"movie_id" INT4 NOT NULL,
	"actor_id" INT4 NOT NULL,
	"character_name" TEXT,
	"billing_order" INT4,
	PRIMARY KEY("movie_id", "actor_id"),
	FOREIGN KEY ("movie_id") REFERENCES "movies"("id"),
	FOREIGN KEY ("actor_id") REFERENCES "actors"("id")
);

