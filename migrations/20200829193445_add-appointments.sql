CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE "appointments"
(
 "id"               uuid DEFAULT uuid_generate_v4(),
 "name"             varchar NOT NULL,
 "description"      varchar NOT NULL,
 "start_date"       TIMESTAMP NULL,
 "end_date"         TIMESTAMP NULL,
 "notes"            text NULL,
 "meeting_partners" json NULL,
 "client_attendees" json NOT NULL,
 "user_id"      uuid NOT NULL,
 "is_completed"     boolean NOT NULL default false,
 "created_at"       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

 CONSTRAINT "PK_appointments" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_user" FOREIGN KEY ( "user_id" ) REFERENCES "users" ( "id" )
);

CREATE INDEX "fkIdx_user_on_appointments" ON "appointments"
(
 "user_id"
);







