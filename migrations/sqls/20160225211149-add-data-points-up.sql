CREATE TABLE data_points (
    id SERIAL PRIMARY KEY, 
    data JSONB, 
    agent_id VARCHAR(36),
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
