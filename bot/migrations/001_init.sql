CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    protocol TEXT NOT NULL,
    network TEXT NOT NULL,
    asset_address TEXT NOT NULL,
    amount NUMERIC NOT NULL,
    apy NUMERIC,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

CREATE TABLE apy_snapshots (
    id SERIAL PRIMARY KEY,
    added_at TIMESTAMP DEFAULT now(),
    protocol TEXT NOT NULL,
    network TEXT NOT NULL,
    AaveAsset TEXT NOT NULL,
    apy NUMERIC
);
