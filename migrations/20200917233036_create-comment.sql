CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "comments"
(
 "id"                uuid DEFAULT uuid_generate_v4(),
 "content"           varchar NOT NULL,
 "attachment_url"    json NULL,
 "id_user"           uuid NULL,
 "id_employee"       uuid NULL,
 "id_task"           uuid NULL,
 "created_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization"   uuid NULL,

 CONSTRAINT "PK_comment" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_user" FOREIGN KEY ( "id_user" ) REFERENCES "users" ( "id" ),
 CONSTRAINT "FK_employee" FOREIGN KEY ( "id_employee" ) REFERENCES "employees" ( "id" ),
 CONSTRAINT "FK_task" FOREIGN KEY ( "id_task" ) REFERENCES "tasks" ( "id" )
 CONSTRAINT "FK_organization" FOREIGN KEY ( "id_organization" ) REFERENCES "organizations" ( "id" )
);

CREATE INDEX "fkIdx_users_on_comments" ON "comments"
(
 "id_user"
);

CREATE INDEX "fkIdx_employees_on_comments" ON "comments"
(
 "id_employee"
);

CREATE INDEX "fkIdx_task_on_comments" ON "comments"
(
 "id_task"
);

CREATE INDEX "fkIdx_organization_on_comments" ON "comments"
(
 "id_organization"
);
