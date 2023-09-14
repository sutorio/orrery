CREATE TABLE IF NOT EXISTS user (
  user_id INTEGER NOT NULL PRIMARY KEY,
  user_name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS celestial_body (
  body_id INTEGER NOT NULL PRIMARY KEY,
  body_name TEXT NOT NULL UNIQUE
);