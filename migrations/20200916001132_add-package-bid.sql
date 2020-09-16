CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE status_tag_enum AS ENUM ('Open', 'On hold', 'close');

CREATE TABLE "bids"
(
 "id"                       uuid DEFAULT uuid_generate_v4(),
 "title"                    varchar NOT NULL,
 "attachments_url"           json NULL,
 "status"                   status_tag_enum  NOT NULL DEFAULT 'Open'
 "due_date"                 TIMESTAMP NULL,
 "primary_bidding_contact"  uuid NOT NULL,
 "bidding_cc_list"          json NULL,
 "project_information"      text NOT NULL,
 "id_proposal"              uuid NOT NULL,
 "created_at"               TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"               TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization"          uuid NULL,

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
