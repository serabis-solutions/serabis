CREATE TABLE agents (
    id SERIAL PRIMARY KEY, 
    key VARCHAR(36) UNIQUE NOT NULL,
    hostname VARCHAR(128),
    shortname VARCHAR(32),
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

ALTER TABLE data_points RENAME COLUMN agent_id TO agent_key;
ALTER TABLE data_points ADD FOREIGN KEY (agent_key) REFERENCES agents(key);
