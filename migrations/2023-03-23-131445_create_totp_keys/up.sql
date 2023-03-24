-- Your SQL goes here

CREATE TABLE totp_keys (
  id INTEGER PRIMARY KEY NOT NULL,
  taget VARCHAR NOT NULL,
  secret_key VARCHAR NOT NULL
)
