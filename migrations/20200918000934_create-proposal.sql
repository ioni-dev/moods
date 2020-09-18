CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE status_tag_enum_for_proposal AS ENUM ('Approved', 'On hold', 'Declined', 'Closed');


CREATE TABLE "proposals"
(
 "id"              uuid DEFAULT uuid_generate_v4(),
 "name"            varchar NOT NULL,
 "status"          status_tag_enum_for_proposal NULL,
 "introduction"    text NOT NULL,
 "attachment_path" jsonb NULL,
 "estimate"        jsonb NOT NULL,
 "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_contact"      uuid NULL,
 "id_user"         uuid NULL,

 CONSTRAINT "PK_proposal" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_contact" FOREIGN KEY ( "id_contact" ) REFERENCES "contacts" ( "id" ),
 CONSTRAINT "FK_user" FOREIGN KEY ( "id_user" ) REFERENCES "users" ( "id" )
);

CREATE INDEX "fkIdx_contact_on_proposals" ON "proposals"
(
 "id_contact"
);

CREATE INDEX "fkIdx_user_on_proposals" ON "proposals"
(
 "id_user"
);
