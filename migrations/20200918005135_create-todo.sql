CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "todos"
(
 "id"              uuid DEFAULT uuid_generate_v4(),
 "title"           varchar NOT NULL,
 "description"     varchar NOT NULL,
 "assigned_to"     jsonb NULL  ,
 "due_date"        DATE NULL,
 "location"        jsonb NULL  ,
 "attachment_path" jsonb NULL  ,
 "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_project"      uuid  NULL,
 "id_user"         uuid NULL,

 CONSTRAINT "PK_todo" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_user" FOREIGN KEY ( "id_user" ) REFERENCES "users" ( "id" )
);

CREATE INDEX "fkIdx_project_on_todos" ON "todos"
(
 "id_project"
);

CREATE INDEX "fkIdx_assigned_to_on_todos" ON "todos"
(
 "id_user"
);

