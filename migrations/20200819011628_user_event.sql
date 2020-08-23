CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE user_events (
    id uuid DEFAULT uuid_generate_v4() primary key,
    CONSTRAINT fk_user FOREIGN KEY(id) REFERENCES users(id),
    CONSTRAINT fk_event FOREIGN KEY(id) REFERENCES events(id)
);
