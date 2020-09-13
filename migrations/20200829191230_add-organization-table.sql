CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "organizations"
(
 "id"             uuid DEFAULT uuid_generate_v4(),
 "name"           varchar NOT NULL,
 "address"        varchar NOT NULL,
 "website"        varchar NULL,
 "email"          varchar NULL,
 "business_type"  varchar NULL,
 "tag"            varchar NULL,
 "active"         boolean NOT NULL DEFAULT true,
 "phone"          varchar NULL,
 "created_at"     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,
 "updated_at"     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

 CONSTRAINT "PK_organization" PRIMARY KEY ( "id" )
);







