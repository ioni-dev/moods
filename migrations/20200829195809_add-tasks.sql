CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE status_tag_enum_for_tasks AS ENUM ('active', 'on hold', 'completed');

CREATE TABLE "tasks"
(
 "id"                uuid DEFAULT uuid_generate_v4(),
 "name"              varchar NOT NULL,
 "status"            status_tag_enum_for_tasks NOT NULL default 'active',
 "assigned"          json NULL,
 "description"       varchar NULL,
 "location"          json NULL,
 "attachments_path"  json NULL,
 "due_date"          date NULL,
 "id_comment"        uuid NULL,
 "created_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization"   uuid NULL,
 "id_project"        uuid NULL,

 CONSTRAINT "PK_task" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_organization" FOREIGN KEY ( "id_organization" ) REFERENCES "organizations" ( "id" )
 CONSTRAINT "FK_project" FOREIGN KEY ( "id_project" ) REFERENCES "projects" ( "id" )
);

CREATE INDEX "fkIdx_organization_on_tasks" ON "public"."tasks"
(
 "id_organization"
);

CREATE INDEX "fkIdx_project_on_tasks" ON "public"."tasks"
(
 "id_project"
);

CREATE INDEX "fkIdx_comment_on_tasks" ON "public"."tasks"
(
 "id_comment"
);



