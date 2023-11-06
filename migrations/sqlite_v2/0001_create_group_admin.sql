CREATE TABLE usergroup_admin (
       usergroup INTEGER REFERENCES usergroup (id) ON DELETE CASCADE,
       session INTEGER REFERENCES session (id) ON DELETE CASCADE,
       PRIMARY KEY (usergroup, session)
);

CREATE INDEX usergroup_admin_usergroup_idx ON usergroup_admin (usergroup);
CREATE INDEX usergroup_admin_session_idx ON usergroup_admin (session);
