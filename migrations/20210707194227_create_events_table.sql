-- Add migration script here
CREATE TABLE IF NOT EXISTS events (
    id              INTEGER     PRIMARY KEY, --SQLite aliases this to row id
    name            TEXT        NOT NULL,
    description     TEXT        NOT NULL,
    latitude        REAL        NOT NULL,
    longitude       REAL        NOT NULL,
    start_time      DATETIME    NOT NULL,
    end_time        DATETIME    NOT NULL,
    timestamp       DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP
    --TODO: Created_By when users are implemented
);