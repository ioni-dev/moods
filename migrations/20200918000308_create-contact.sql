CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "contacts"
(
 "id"                uuid DEFAULT uuid_generate_v4(),
 "first_name"        varchar NOT NULL,
 "middle_name"       varchar NULL ,
 "last_name"         varchar NOT NULL,
 "email"             varchar NOT NULL UNIQUE,
 "cell_phone_number" varchar NULL ,
 "linkedin"          varchar NULL ,
 "facebook"          varchar NULL ,
 "twitter"           varchar NULL ,
 "website"           varchar NULL ,
 "is_active"         boolean NOT NULL default true,
 "last_talked_to"    TIMESTAMP NULL,
 "birthday"          date NULL,
 "company_name"      varchar NULL ,
 "company_website"   varchar NULL ,
 "position"          varchar NULL ,
 "logs"              varchar NULL ,
 "work_phone"        varchar NULL ,
 "pic_url"           varchar NULL ,
 "last_consulted_at" TIMESTAMP NULL,
 "created_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization"   uuid NULL,
 "id_user"           uuid NULL,

 CONSTRAINT "PK_contact" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_organization" FOREIGN KEY ( "id_organization" ) REFERENCES "organizations" ( "id" )
);

CREATE INDEX "fkIdx_organization_on_contacts" ON "contacts"
(
 "id_organization"
);

CREATE INDEX "fkIdx_user_on_contacts" ON "contacts"
(
 "id_user"
);
