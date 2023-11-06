CREATE TABLE usergroup_admin (
       usergroup INTEGER REFERENCES usergroup (id) ON DELETE CASCADE,
       session INTEGER REFERENCES session (id) ON DELETE CASCADE,
       PRIMARY KEY (usergroup, session)
);

CREATE INDEX ON usergroup_admin (usergroup);
CREATE INDEX ON usergroup_admin (session);
