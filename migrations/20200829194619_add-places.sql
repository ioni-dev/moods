CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "places"
(
 "id"          uuid DEFAULT uuid_generate_v4(),
 "street"      varchar NOT NULL,
 "city"        varchar NOT NULL,
 "postal_code" varchar NOT NULL,
 "country"     varchar NOT NULL,
 "state"       varchar NOT NULL,
 "latitude"    double precision NULL,
 "longitude"   double precision NULL,
 "created_at"  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "event_id"    uuid  NULL,

 CONSTRAINT "PK_place" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_event_on_places" FOREIGN KEY ( "event_id" ) REFERENCES "public"."events" ( "id" )
);

CREATE INDEX "fkIdx_event_on_places" ON "public"."places"
(
 "event_id"
);
