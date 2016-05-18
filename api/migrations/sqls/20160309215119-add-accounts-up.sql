CREATE TABLE accounts (
    id SERIAL PRIMARY KEY, 
    key VARCHAR(36) UNIQUE NOT NULL,
    name VARCHAR(128),
    shortname VARCHAR(32),
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

ALTER TABLE agents ADD account_key VARCHAR(36) UNIQUE NOT NULL;
ALTER TABLE agents ADD FOREIGN KEY (account_key) REFERENCES accounts(key);
