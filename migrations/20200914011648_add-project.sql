CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "projects"
(
 "id"                uuid DEFAULT uuid_generate_v4(),
 "name"              varchar NOT NULL,
 "attachment_url"    json NULL,
 "start_date"        TIMESTAMP NULL DEFAULT now(),
 "due_date"          TIMESTAMP NULL,
 "id_contact"        uuid NULL,
 "id_appointment"    uuid NULL,
 "id_user"           uuid NULL,
 "id_employee"       uuid NULL,
 "id_task"           uuid NULL,
 "created_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization"   uuid NULL,

 CONSTRAINT "PK_project" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_user" FOREIGN KEY ( "id_user" ) REFERENCES "users" ( "id" )
 CONSTRAINT "FK_employee" FOREIGN KEY ( "id_employee" ) REFERENCES "employees" ( "id" )
 CONSTRAINT "FK_task" FOREIGN KEY ( "id_task" ) REFERENCES "tasks" ( "id" )
 CONSTRAINT "FK_contact" FOREIGN KEY ( "id_contact" ) REFERENCES "contacts" ( "id" )
 CONSTRAINT "FK_organization" FOREIGN KEY ( "id_organization" ) REFERENCES "organizations" ( "id" )
 CONSTRAINT "FK_appointment" FOREIGN KEY ( "id_appointment" ) REFERENCES "appointments" ( "id" )
);

CREATE INDEX "fkIdx_users_on_projects" ON "public"."projects"
(
 "id_user"
);

CREATE INDEX "fkIdx_employees_on_projects" ON "public"."projects"
(
 "id_employee"
);

CREATE INDEX "fkIdx_task_on_projects" ON "public"."projects"
(
 "id_task"
);

CREATE INDEX "fkIdx_contact_projects" ON "public"."projects"
(
 "id_contact"
);

CREATE INDEX "fkIdx_organization_projects" ON "public"."projects"
(
 "id_organization"
);

CREATE INDEX "fkIdx_appointment_projects" ON "public"."projects"
(
 "id_appointment"
);
