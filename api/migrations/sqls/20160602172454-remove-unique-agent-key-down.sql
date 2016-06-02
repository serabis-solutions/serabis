ALTER TABLE agents DROP CONSTRAINT agents_account_key_key;
CREATE UNIQUE INDEX agents_account_key_key ON agents (account_key);
