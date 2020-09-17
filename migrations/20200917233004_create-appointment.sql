CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE "appointments"
(
 "id"               uuid DEFAULT uuid_generate_v4(),
 "name"             varchar NOT NULL,
 "description"      varchar NOT NULL,
 "start_date"       TIMESTAMP NULL,
 "end_date"         TIMESTAMP NULL,
 "meeting_partners" json NULL,
 "client_attendees" json NULL,
 "is_completed"     boolean NOT NULL default false,
 "created_at"       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_user"          uuid NULL,
 "id_note"          uuid NUll,
 "id_project"       uuid NULL,
 "id_lead"          uuid NULL,
 "id_contact"       uuid NULL,

 CONSTRAINT "PK_appointment" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_user" FOREIGN KEY ( "id_user" ) REFERENCES "users" ( "id" ),
 CONSTRAINT "FK_note" FOREIGN KEY ( "id_note" ) REFERENCES "notes" ( "id" ),
 CONSTRAINT "FK_project" FOREIGN KEY ( "id_project" ) REFERENCES "projects" ( "id" ),
 CONSTRAINT "FK_lead" FOREIGN KEY ( "id_lead" ) REFERENCES "leads" ( "id" ),
 CONSTRAINT "FK_contact" FOREIGN KEY ( "id_contact" ) REFERENCES "contacts" ( "id" )
);

CREATE INDEX "fkIdx_user_on_appointments" ON "appointments"
(
 "id_user"
);

CREATE INDEX "fkIdx_note_on_appointments" ON "appointments"
(
 "id_note"
);

CREATE INDEX "fkIdx_project_on_appointments" ON "appointments"
(
 "id_project"
);

CREATE INDEX "fkIdx_lead_on_appointments" ON "appointments"
(
 "id_lead"
);

CREATE INDEX "fkIdx_contact_on_appointments" ON "appointments"
(
 "id_contact"
);

