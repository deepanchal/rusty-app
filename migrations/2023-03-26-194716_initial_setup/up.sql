-- Base fields:
-- CREATE TABLE IF NOT EXISTS base (
--     id SERIAL PRIMARY KEY,
--     created_at TIMESTAMP NOT NULL DEFAULT NOW (),
--     updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
--     invalidated_at TIMESTAMP,
--     created_by INTEGER REFERENCES user_account (id),
--     updated_by INTEGER REFERENCES user_account (id),
-- );
CREATE TABLE IF NOT EXISTS user_account (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    invalidated_at TIMESTAMP,
    created_by INTEGER REFERENCES user_account (id),
    updated_by INTEGER REFERENCES user_account (id),
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL
);
CREATE TABLE IF NOT EXISTS person (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    invalidated_at TIMESTAMP,
    created_by INTEGER REFERENCES user_account (id),
    updated_by INTEGER REFERENCES user_account (id),
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    user_id INTEGER REFERENCES user_account (id)
);
CREATE TABLE IF NOT EXISTS todo (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    invalidated_at TIMESTAMP,
    created_by INTEGER REFERENCES user_account (id),
    updated_by INTEGER REFERENCES user_account (id),
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    done BOOLEAN NOT NULL,
    user_id INTEGER REFERENCES user_account (id)
);