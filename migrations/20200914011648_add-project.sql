CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "projects"
(
 "id"                uuid DEFAULT uuid_generate_v4(),
 "name"              varchar NOT NULL,
 "attachment_url"    json NULL,
 "start_date"        TIMESTAMP NULL DEFAULT now(),
 "due_date"          TIMESTAMP NULL,

 "id_employee"       uuid NULL,
 "id_task"           uuid NULL,
 "created_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization"   uuid NULL,

 CONSTRAINT "PK_comment" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_user" FOREIGN KEY ( "id_user" ) REFERENCES "users" ( "id" )
 CONSTRAINT "FK_employee" FOREIGN KEY ( "id_employee" ) REFERENCES "employees" ( "id" )
 CONSTRAINT "FK_task" FOREIGN KEY ( "id_task" ) REFERENCES "tasks" ( "id" )
);

CREATE INDEX "fkIdx_users_on_comments" ON "public"."comments"
(
 "id_user"
);

CREATE INDEX "fkIdx_employees_on_comments" ON "public"."comments"
(
 "FK_employee"
);

CREATE INDEX "fkIdx_task_on_comments" ON "public"."comments"
(
 "FK_task"
);
