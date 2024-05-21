CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       email VARCHAR NOT NULL UNIQUE,
                       password VARCHAR NOT NULL,
                       reset_password_token VARCHAR,
                       reset_password_sent_at TIMESTAMP,
                       sign_in_count INT DEFAULT 0 NOT NULL,
                       current_sign_in_at TIMESTAMP,
                       last_sign_in_at TIMESTAMP,
                       current_sign_in_ip INET,
                       last_sign_in_ip INET,
                       confirmation_token VARCHAR,
                       confirmation_sent_at TIMESTAMP,
                       confirmed_at TIMESTAMP,
                       created_at TIMESTAMP NOT NULL,
                       updated_at TIMESTAMP NOT NULL
)