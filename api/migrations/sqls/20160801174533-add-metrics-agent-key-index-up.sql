BEGIN;
    CREATE INDEX "metrics-agent_key" ON metrics (agent_key);
    CREATE INDEX "metrics-agent_key-data_type" ON metrics (agent_key, (data->>'type'));
COMMIT;
