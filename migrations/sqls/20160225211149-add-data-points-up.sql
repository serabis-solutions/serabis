CREATE TABLE data_points (
    id SERIAL PRIMARY KEY, 
    data JSONB, 
    agent_id VARCHAR(32),
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
