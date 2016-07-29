CREATE TABLE contacts (
    id SERIAL PRIMARY KEY, 
    account_id INTEGER NOT NULL,
    fname VARCHAR(128),
    lname VARCHAR(128),
    email VARCHAR(254), --Limited by RFC5322
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
