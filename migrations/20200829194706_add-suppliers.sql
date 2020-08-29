CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE status_tag_enum AS ENUM ('active', 'pending', 'cancelled');

CREATE TABLE "suppliers"
(
 "id"           uuid DEFAULT uuid_generate_v4(),
 "name"         varchar NOT NULL,
 "contact_info" json NOT NULL,
 "address"      varchar NOT NULL,
 "type"         varchar NOT NULL,
 "website"      varchar(50) NOT NULL,
 "status_tag"   status_tag_enum  NULL,
 "organization_id" uuid NOT NULL,
 "event_id"       uuid NOT NULL,

 CONSTRAINT "PK_suppliers" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_organization" FOREIGN KEY ( "organization_id" ) REFERENCES "organizations" ( "id" ),
 CONSTRAINT "FK_event" FOREIGN KEY ( "event_id" ) REFERENCES "events" ( "id" )
);

CREATE INDEX "fkIdx_organization_on_suppliers" ON "public"."suppliers"
(
 "organization_id"
);

CREATE INDEX "fkIdx_event_on_suppliers" ON "public"."suppliers"
(
 "event_id"
);
