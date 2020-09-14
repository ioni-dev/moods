CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "emails"
(
 "id"                uuid DEFAULT uuid_generate_v4(),
 "url"               varchar NOT NULL,
 "id_employee"       uuid NULL,
 "id_task"           uuid NULL,
 "id_user"           uuid NULL,
 "id_contact"        uuid NULL,
 "created_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

 CONSTRAINT "PK_email" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_user" FOREIGN KEY ( "id_user" ) REFERENCES "users" ( "id" )
 CONSTRAINT "FK_employee" FOREIGN KEY ( "id_employee" ) REFERENCES "employees" ( "id" )
 CONSTRAINT "FK_task" FOREIGN KEY ( "id_task" ) REFERENCES "tasks" ( "id" )
 CONSTRAINT "FK_contact" FOREIGN KEY ( "id_contact" ) REFERENCES "contacts" ( "id" )
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

CREATE INDEX "fkIdx_contact_on_comments" ON "public"."comments"
(
 "FK_contact"
);
