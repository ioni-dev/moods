CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE status_tag_enum AS ENUM ('Open', 'On hold', 'close');

CREATE TABLE "bids"
(
 "id"                       uuid DEFAULT uuid_generate_v4(),
 "title"                    varchar NOT NULL,
 "attachments_url"          json NULL,
 "status"                   status_tag_enum  NOT NULL DEFAULT 'Open'
 "due_date"                 TIMESTAMP NULL,
 "primary_bidding_contact"  uuid NOT NULL,
 "bidding_cc_list"          json NULL,
 "project_information"      text NOT NULL,
 "id_proposal"              uuid NOT NULL,
 "created_at"               TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "updated_at"               TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 "id_organization"          uuid NULL,

 CONSTRAINT "PK_bids" PRIMARY KEY ( "id" ),
 CONSTRAINT "FK_primary_bidding_contact" FOREIGN KEY ( "primary_bidding_contact" ) REFERENCES "users" ( "id" )
 CONSTRAINT "FK_proposal" FOREIGN KEY ( "id_proposal" ) REFERENCES "proposals" ( "id" )
 CONSTRAINT "FK_organization" FOREIGN KEY ( "id_organization" ) REFERENCES "organizations" ( "id" )
);

CREATE INDEX "fkIdx_users_on_bids" ON "public"."bids"
(
 "primary_bidding_contact"
);

CREATE INDEX "fkIdx_proposal_on_bids" ON "public"."bids"
(
 "id_proposal"
);

CREATE INDEX "fkIdx_organization_on_bids" ON "public"."bids"
(
 "id_organization"
);