--Conditions top level definition of a condition
CREATE TABLE conditions (
    id SERIAL PRIMARY KEY, 
    account_id INTEGER NOT NULL,
    name VARCHAR(128),
    shortname VARCHAR(32),
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
ALTER TABLE conditions ADD FOREIGN KEY (account_id) REFERENCES accounts(id);

--Condition Components the parts of a condition to allow multi-part conditions
CREATE TABLE condition_components (
    id SERIAL PRIMARY KEY,
    condition_id INTEGER NOT NULL,
    type VARCHAR(64) NOT NULL,
    opperator VARCHAR(4) NOT NULL, --There's probably something clever we should do here
    trigger_value VARCHAR(64), --Seems massively wasteful but we might want string comparrison?
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
ALTER TABLE condition_components ADD FOREIGN KEY (condition_id) REFERENCES conditions(id);

--Agent Conditions The connection between agents and conditions. Maintains the current state
--and when it last changed
CREATE TABLE agent_conditions(
    id SERIAL PRIMARY KEY,
    agent_id INTEGER NOT NULL,
    condition_id INTEGER NOT NULL,
    triggered BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
ALTER TABLE agent_conditions ADD FOREIGN KEY (agent_id) REFERENCES agents(id);
ALTER TABLE agent_conditions ADD FOREIGN KEY (condition_id) REFERENCES conditions(id);
