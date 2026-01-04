CREATE TABLE apy_snapshots (
    id SERIAL PRIMARY KEY,
    added_at TIMESTAMP DEFAULT now(),
    protocol TEXT NOT NULL,
    network TEXT NOT NULL,
    asset TEXT NOT NULL,
    apy DOUBLE PRECISION
);
