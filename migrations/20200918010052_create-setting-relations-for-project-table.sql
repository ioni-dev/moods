ALTER TABLE tasks
    ADD CONSTRAINT FK_project FOREIGN KEY (id_project) REFERENCES projects (id);

ALTER TABLE tasks
    ADD CONSTRAINT FK_comment FOREIGN KEY (id_comment) REFERENCES comments (id);

ALTER TABLE appointments
    ADD CONSTRAINT FK_project FOREIGN KEY (id_project) REFERENCES projects (id);

ALTER TABLE contacts
    ADD CONSTRAINT FK_user FOREIGN KEY (id_user) REFERENCES users (id);

ALTER TABLE notes
    ADD CONSTRAINT FK_contact  FOREIGN KEY ( id_contact ) REFERENCES contacts ( id );

ALTER TABLE notes
    ADD CONSTRAINT FK_lead  FOREIGN KEY ( id_lead ) REFERENCES leads ( id );

ALTER TABLE notes
    ADD CONSTRAINT FK_task  FOREIGN KEY ( id_task ) REFERENCES tasks ( id );

ALTER TABLE notes
    ADD CONSTRAINT FK_bid  FOREIGN KEY ( id_bid ) REFERENCES bids ( id );

ALTER TABLE notes
    ADD CONSTRAINT FK_appointment FOREIGN KEY ( id_appointment ) REFERENCES appointments ( id );

ALTER TABLE notes
    ADD CONSTRAINT FK_project FOREIGN KEY ( id_project ) REFERENCES projects ( id );

ALTER TABLE notes
    ADD CONSTRAINT FK_user FOREIGN KEY ( id_user ) REFERENCES users ( id );


