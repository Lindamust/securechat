-- Add migration script here
CREATE DOMAIN bytes32 AS bytea CHECK (octet_length(VALUE) = 32);
CREATE DOMAIN bytes64 AS bytea CHECK (octet_length(VALUE) = 64);

CREATE TABLE IF NOT EXISTS users (
    id                  UUID            PRIMARY KEY DEFAULT uuidv7(),
    username            TEXT            NOT NULL UNIQUE,
    email               TEXT            NULL,
    password_hash       TEXT            NOT NULL,
    ik_pub              bytes32         NOT NULL UNIQUE,
    ik_pub_ed           bytes32         NOT NULL UNIQUE,
    spk_pub             bytes32         NOT NULL,
    spk_pub_sig         bytes64         NOT NULL,
    spk_uploaded_at     TIMESTAMPTZ     NOT NULL
);

CREATE TABLE IF NOT EXISTS otpks (
    id          UUID        PRIMARY KEY DEFAULT uuidv7(),
    user_id     UUID        NOT NULL REFERENCES users(id),
    otpk_pub    bytes32     NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS auth_challenges (
    nonce           bytes32         PRIMARY KEY,
    user_id         UUID            NOT NULL REFERENCES users(id),
    expires_at      TIMESTAMPTZ     NOT NULL
);

CREATE INDEX IF NOT EXISTS otpk_user_idx ON otpks(user_id);
CREATE UNIQUE INDEX IF NOT EXISTS auth_user_idx ON auth_challenges(user_id);
