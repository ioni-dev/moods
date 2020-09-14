CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "project"
(
 "id"              uuid DEFAULT uuid_generate_v4(),
 "name"            varchar NOT NULL,
 "description"     varchar NOT NULL,
 "category_tag"    varchar NOT NULL,
 "active"          boolean NOT NULL default true,
 "start_date"      date NULL,
 "end_date"        date NULL,
 "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "event_id"        uuid  NULL,
 "organization_id" uuid NOT NULL,

 CONSTRAINT "PK_event" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_organization" FOREIGN KEY ( "organization_id" ) REFERENCES "organizations" ( "id" )
);

CREATE INDEX "fkIdx_organization_on_events" ON "public"."events"
(
 "organization_id"
);
