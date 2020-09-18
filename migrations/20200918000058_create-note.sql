CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "notes"
(
 "id"              uuid DEFAULT uuid_generate_v4(),
 "title"           varchar NOT NULL,
 "content"         TEXT NOT NULL,
 "attachment_path" jsonb NULL,
 "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

 CONSTRAINT "PK_note" PRIMARY KEY ( "id" )
);


