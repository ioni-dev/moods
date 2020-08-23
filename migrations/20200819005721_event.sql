CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE event_type AS ENUM ('travel', 'wedding', 'conference', 'other');

CREATE TABLE events (
    id uuid DEFAULT uuid_generate_v4() primary key,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    category event_type NOT NULL,
    location point,
    date DATE,
    image VARCHAR NULL,
    rules text[],
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
