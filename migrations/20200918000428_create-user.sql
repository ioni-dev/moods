CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users"
(
 "id"              uuid DEFAULT uuid_generate_v4(),
 "name"            varchar NOT NULL,
 "email"           varchar NOT NULL UNIQUE,
 "password_hash"   varchar NOT NULL,
 "email_verified"  boolean NOT NULL default false,
 "active"          boolean NOT NULL default true,
 "max_employees"   int NOT NULL default 1,
 "is_admin"        boolean NOT NULL default true,
 "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization" uuid  NULL,

 CONSTRAINT "PK_user" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_organization" FOREIGN KEY ( "id_organization" ) REFERENCES "organizations" ( "id" )
);

CREATE INDEX "fkIdx_organization_on_users" ON "users"
(
 "id_organization"
);
