CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "employees"
(
 "id"              uuid DEFAULT uuid_generate_v4(),
 "name"            varchar NOT NULL,
 "email"           varchar NOT NULL UNIQUE,
 "password_hash"   varchar NOT NULL,
 "email_verified"  boolean NOT NULL default false,
 "active"          boolean NOT NULL default true,
 "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization" uuid NULL,
 "id_user"         uuid NULL,

 CONSTRAINT "PK_employee" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_organization" FOREIGN KEY ( "id_organization" ) REFERENCES "organizations" ( "id" ),
 CONSTRAINT "FK_user" FOREIGN KEY ( "id_user" ) REFERENCES "users" ( "id" )
);

CREATE INDEX "fkIdx_organization_on_employees" ON "employees"
(
 "id_organization"
);

CREATE INDEX "fkIdx_user_on_employees" ON "employees"
(
 "id_user"
);

