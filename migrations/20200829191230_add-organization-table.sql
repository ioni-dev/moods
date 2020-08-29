CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "organizations"
(
 "id"             uuid DEFAULT uuid_generate_v4(),
 "name"           varchar NOT NULL,
 "address"        varchar NOT NULL,
 "website"        varchar null,
 "email"          varchar NOT NULL UNIQUE,
 "password_hash"  varchar NOT NULL,
 "active"         boolean NOT NULL DEFAULT true,
 "email_verified" boolean NOT NULL DEFAULT false,
 "max_employees"  int NOT NULL,
 "max_users"      int NOT NULL,
 "created_at"     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
 "updated_at"     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "phone"          varchar NOT NULL,
 CONSTRAINT "PK_organization" PRIMARY KEY ( "id" )
);







