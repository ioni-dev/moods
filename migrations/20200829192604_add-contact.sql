CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "contacts"
(
 "id"                uuid DEFAULT uuid_generate_v4(),
 "first_name"        varchar NOT NULL,
 "middle_name"       varchar NULL,
 "last_name"         varchar NOT NULL,
 "phone"             varchar NULL,
 "linkedin"          varchar NULL,
 "facebook"          varchar NULL,
 "twitter"           varchar NULL,
 "website"           varchar NULL,
 "description"       varchar NOT NULL,
 "is_active"         boolean NOT NULL default true,
 "last_talked_to"    TIMESTAMP NULL,
 "birthday"          date NULL,
 "company"           varchar NOT NULL,
 "company_website"   varchar NULL,
 "avatar_url"        varchar NULL,
 "last_consulted_at" date NULL,
 "created_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "organization_id"   uuid  NULL,

 CONSTRAINT "PK_contact" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_organization" FOREIGN KEY ( "organization_id" ) REFERENCES "organizations" ( "id" )
);

CREATE INDEX "fkIdx_organization_on_contacts" ON "contacts"
(
 "organization_id"
);
