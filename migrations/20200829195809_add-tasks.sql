CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE status_tag_enum_for_tasks AS ENUM ('active', 'pending', 'completed');

CREATE TABLE "tasks"
(
 "id"                uuid DEFAULT uuid_generate_v4(),
 "name"              varchar NOT NULL,
 "status"            status_tag_enum_for_tasks NOT NULL default 'pending',
 "description"       varchar NOT NULL,
 "finishing_date"    TIMESTAMP NULL,
 "created_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization"   uuid NULL,

 CONSTRAINT "PK_task" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_organization" FOREIGN KEY ( "id_organization" ) REFERENCES "organizations" ( "id" )
);

CREATE INDEX "fkIdx_organization_on_tasks" ON "public"."tasks"
(
 "id_organization"
);





