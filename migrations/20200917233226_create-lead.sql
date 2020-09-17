CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE stage_tag_enum AS ENUM ('Open', 'On hold', 'close');


CREATE TABLE "leads"
(
 "id"                   uuid DEFAULT uuid_generate_v4(),
 "title"                varchar NOT NULL,
 "stage"                stage_tag_enum NOT NULL,
 "opened_date"          DATE NULL,
 "estimate_sale_date"   DATE NULL,
 "estimate_revenue"     numeric(15,6) NULL,
 "address"              varchar NULL,
 "lat"                  DOUBLE PRECISION NULL,
 "long"                 DOUBLE PRECISION NULL,
 "attachment_path"      JSON NULL,
 "created_at"           TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"           TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_proposal"          uuid NULL,
 "id_note"              uuid NULL,
 "id_contact"           uuid NULL,

 CONSTRAINT "PK_lead" PRIMARY KEY ( "id" )
 CONSTRAINT "FK_proposal" FOREIGN KEY ( "id_proposal" ) REFERENCES "proposals" ( "id" ),
 CONSTRAINT "FK_note" FOREIGN KEY ( "id_note" ) REFERENCES "notes" ( "id" ),
 CONSTRAINT "FK_contact" FOREIGN KEY ( "id_contact" ) REFERENCES "contacts" ( "id" )
);

CREATE INDEX "fkIdx_proposal_on_todos" ON "leads"
(
 "id_proposal"
);

CREATE INDEX "fkIdx_note_on_todos" ON "leads"
(
 "id_note"
);

CREATE INDEX "fkIdx_contact_on_todos" ON "leads"
(
 "id_contact"
);
