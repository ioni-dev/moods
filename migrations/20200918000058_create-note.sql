CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "notes"
(
 "id"              uuid DEFAULT uuid_generate_v4(),
 "title"           varchar NOT NULL,
 "content"         TEXT NOT NULL,
 "attachment_path" jsonb NULL ,
 "created_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_contact"      uuid NULL,
 "id_lead"         uuid NULL,
 "id_task"         uuid NUll,
 "id_bid"          uuid NULL,
 "id_appointment"  uuid NUll,
 "id_project"      uuid NULL,
 "id_user"         uuid NULL,

 CONSTRAINT "PK_note" PRIMARY KEY ( "id" )
);


CREATE INDEX "fkIdx_contact_on_note" ON "notes"
(
 "id_contact"
);


CREATE INDEX "fkIdx_lead_on_note" ON "notes"
(
 "id_lead"
);


CREATE INDEX "fkIdx_task_on_note" ON "notes"
(
 "id_task"
);

CREATE INDEX "fkIdx_bid_on_note" ON "notes"
(
 "id_bid"
);

CREATE INDEX "fkIdx_appointments_on_note" ON "notes"
(
 "id_appointment"
);

CREATE INDEX "fkIdx_project_note" ON "notes"
(
 "id_project"
);

CREATE INDEX "fkIdx_user_note" ON "notes"
(
 "id_user"
);