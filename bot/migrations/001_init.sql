CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    protocol TEXT NOT NULL,
    network TEXT NOT NULL,
    asset TEXT NOT NULL,
    amount NUMERIC NOT NULL,
    apy NUMERIC,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);
