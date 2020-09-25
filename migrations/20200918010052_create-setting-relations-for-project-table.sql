ALTER TABLE tasks
    ADD CONSTRAINT FK_project FOREIGN KEY (id_project) REFERENCES projects (id);

ALTER TABLE tasks
    ADD CONSTRAINT FK_comment FOREIGN KEY (id_comment) REFERENCES comments (id);

ALTER TABLE appointments
    ADD CONSTRAINT FK_project FOREIGN KEY (id_project) REFERENCES projects (id);

ALTER TABLE contacts
    ADD CONSTRAINT FK_user FOREIGN KEY (id_user) REFERENCES users (id);