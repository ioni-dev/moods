CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "todos"
(
 "id"              uuid DEFAULT uuid_generate_v4(),
 "title"           varchar NOT NULL,
 "description"     varchar NOT NULL,
 "assigned_to"     uuid NULL,
 "due_date"        DATE NULL,
 "lat"             DOUBLE PRECISION NULL,
 "long"            DOUBLE PRECISION NULL,
 "attachment_path" JSONT NULL,
 "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_project"      uuid  NULL,
 CONSTRAINT "PK_user" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_project" FOREIGN KEY ( "id_project" ) REFERENCES "projects" ( "id" )
 CONSTRAINT "FK_assigned_to" FOREIGN KEY ( "assigned_to" ) REFERENCES "projects" ( "id" )
);

CREATE INDEX "fkIdx_project_on_todos" ON "todos"
(
 "id_project"
);

CREATE INDEX "fkIdx_project_on_todos" ON "todos"
(
 "assigned_to"
);

