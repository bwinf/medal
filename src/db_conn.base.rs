/*  medal                                                                                                            *\
 *  Copyright (C) 2022  Bundesweite Informatikwettbewerbe, Robert Czechowski                                                            *
 *                                                                                                                   *
 *  This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero        *
 *  General Public License as published  by the Free Software Foundation, either version 3 of the License, or (at    *
 *  your option) any later version.                                                                                  *
 *                                                                                                                   *
 *  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the       *
 *  implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public      *
 *  License for more details.                                                                                        *
 *                                                                                                                   *
 *  You should have received a copy of the GNU Affero General Public License along with this program.  If not, see   *
\*  <http://www.gnu.org/licenses/>.                                                                                  */

impl MedalObject<Connection> for Submission {
    fn save(&mut self, conn: &Connection) {
        match self.get_id() {
            Some(_id) => unimplemented!(),
            None => {
                let query = "INSERT INTO submission (task, session, grade, validated, nonvalidated_grade,
                                                     subtask_identifier, value, date, needs_validation)
                             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";
                conn.execute(query,
                             &[&self.task,
                               &self.user,
                               &self.grade,
                               &self.validated,
                               &self.nonvalidated_grade,
                               &self.subtask_identifier,
                               &self.value,
                               &self.date,
                               &self.needs_validation])
                    .unwrap();
                self.set_id(conn.get_last_id().unwrap());
            }
        }
    }
}

impl MedalObject<Connection> for Participation {
    fn save(&mut self, conn: &Connection) {
        let query = "INSERT INTO participation (contest, session, start_date)
                     VALUES ($1, $2, $3)";
        conn.execute(query, &[&self.contest, &self.user, &self.start]).unwrap();
    }
}

impl MedalObject<Connection> for Group {
    fn save(&mut self, conn: &Connection) {
        match self.get_id() {
            Some(_id) => unimplemented!(),
            None => {
                let query = "INSERT INTO usergroup (name, groupcode, tag, admin, group_created)
                             VALUES ($1, $2, $3, $4, $5)";
                let now = time::get_time();
                println!("{:?}", now);
                conn.execute(query, &[&self.name, &self.groupcode, &self.tag, &self.admin, &now]).unwrap();
                self.set_id(conn.get_last_id().unwrap());
            }
        }
    }
}

impl MedalObject<Connection> for Task {
    fn save(&mut self, conn: &Connection) {
        let query = "SELECT id
                     FROM task
                     WHERE taskgroup = $1
                     AND location = $2";
        conn.query_map_one(query, &[&self.taskgroup, &self.location], |row| row.get(0))
            .unwrap_or(None)
            .map(|id| {
                self.set_id(id);
            })
            .unwrap_or(()); // Err means no entry yet and is expected result

        let id = match self.get_id() {
            Some(id) => {
                let query = "UPDATE task
                             SET taskgroup = $1, location = $2, language = $3, stars = $4
                             WHERE id = $5";
                conn.execute(query, &[&self.taskgroup, &self.location, &self.language, &self.stars, &id]).unwrap();
                id
            }
            None => {
                let query = "INSERT INTO task (taskgroup, location, language, stars)
                             VALUES ($1, $2, $3, $4)";
                conn.execute(query, &[&self.taskgroup, &self.location, &self.language, &self.stars]).unwrap();
                conn.get_last_id().unwrap()
            }
        };
        self.set_id(id);
    }
}

impl MedalObject<Connection> for Taskgroup {
    fn save(&mut self, conn: &Connection) {
        if let Some(first_task) = self.tasks.get(0) {
            let query = "SELECT taskgroup.id
                         FROM taskgroup
                         JOIN task
                         ON task.taskgroup = taskgroup.id
                         WHERE contest = $1
                         AND task.location = $2";
            conn.query_map_one(query, &[&self.contest, &first_task.location], |row| row.get(0))
                .unwrap_or(None)
                .map(|id| {
                    self.set_id(id);
                })
                .unwrap_or(()); // Err means no entry yet and is expected result
        }

        let id = match self.get_id() {
            Some(id) => {
                let query = "UPDATE taskgroup
                             SET contest = $1, name = $2, active = $3, positionalnumber = $4
                             WHERE id = $5";
                conn.execute(query, &[&self.contest, &self.name, &self.active, &self.positionalnumber, &id]).unwrap();
                id
            }
            None => {
                let query = "INSERT INTO taskgroup (contest, name, active, positionalnumber)
                             VALUES ($1, $2, $3, $4)";
                conn.execute(query, &[&self.contest, &self.name, &self.active, &self.positionalnumber]).unwrap();
                conn.get_last_id().unwrap()
            }
        };
        self.set_id(id);
        for task in &mut self.tasks {
            task.taskgroup = id;
            task.save(conn);
        }
    }
}

impl MedalObject<Connection> for Contest {
    fn save(&mut self, conn: &Connection) {
        let query = "SELECT id
                     FROM contest
                     WHERE location = $1
                     AND filename = $2";
        conn.query_map_one(query, &[&self.location, &self.filename], |row| row.get(0))
            .unwrap_or(None)
            .map(|id| {
                self.set_id(id);
            })
            .unwrap_or(()); // Err means no entry yet and is expected result

        let id = match self.get_id() {
            Some(id) => {
                let query = "DELETE FROM contest_tags
                             WHERE id = $1";
                conn.execute(query, &[&id]).unwrap();

                let query = "UPDATE contest
                             SET location = $2,filename = $3, name = $4, duration = $5, public = $6, start_date = $7,
                                 end_date = $8, review_start_date = $9, review_end_date = $10, min_grade = $11,
                                 max_grade = $12, positionalnumber = $13, protected = $14, requires_login = $15,
                                 requires_contest = $16, secret = $17, message = $18, image = $19, language = $20,
                                 category = $21, standalone_task = $22
                             WHERE id = $1";
                conn.execute(query,
                             &[&id,
                               &self.location,
                               &self.filename,
                               &self.name,
                               &self.duration,
                               &self.public,
                               &self.start,
                               &self.end,
                               &self.review_start,
                               &self.review_end,
                               &self.min_grade,
                               &self.max_grade,
                               &self.positionalnumber,
                               &self.protected,
                               &self.requires_login,
                               &self.requires_contest,
                               &self.secret,
                               &self.message,
                               &self.image,
                               &self.language,
                               &self.category,
                               &self.standalone_task])
                    .unwrap();
                id
            }
            None => {
                let query = "INSERT INTO contest (location, filename, name, duration, public, start_date, end_date,
                                                  review_start_date, review_end_date, min_grade, max_grade,
                                                  positionalnumber, protected, requires_login, requires_contest, secret,
                                                  message, image, language, category, standalone_task)
                             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)";
                conn.execute(query,
                             &[&self.location,
                               &self.filename,
                               &self.name,
                               &self.duration,
                               &self.public,
                               &self.start,
                               &self.end,
                               &self.review_start,
                               &self.review_end,
                               &self.min_grade,
                               &self.max_grade,
                               &self.positionalnumber,
                               &self.protected,
                               &self.requires_login,
                               &self.requires_contest,
                               &self.secret,
                               &self.message,
                               &self.image,
                               &self.language,
                               &self.category,
                               &self.standalone_task])
                    .unwrap();
                conn.get_last_id().unwrap()
            }
        };
        self.set_id(id);

        if self.tags.len() > 0 {
            let tagstring = self.tags.join(",");
            let query = "INSERT INTO contest_tags (id, tags)
                         VALUES ($1, $2)";
            conn.execute(query, &[&id, &tagstring]).unwrap();
        }

        for taskgroup in &mut self.taskgroups {
            taskgroup.contest = id;
            taskgroup.save(conn);
        }
        {
            use std::io::Write;
            print!(",");
            std::io::stdout().flush().unwrap();
        }
    }
}

impl MedalConnection for Connection {
    fn reconnect(config: &config::Config) -> Self { Self::reconnect_concrete(config) }

    fn dbtype(&self) -> &'static str { "postgres" }

    fn migration_already_applied(&self, name: &str) -> bool {
        let create_string = "CREATE TABLE IF NOT EXISTS migrations (name TEXT PRIMARY KEY);";
        self.execute(create_string, &[]).unwrap();

        let query = "SELECT name FROM migrations WHERE name = $1";
        self.exists(query, &[&name])
    }

    fn apply_migration(&mut self, name: &str, contents: &str) {
        print!("Applying migration `{}` … ", name);

        let tx = self.transaction().unwrap();

        tx.batch_execute(&contents).unwrap();
        tx.execute("INSERT INTO migrations (name) VALUES ($1)", &[&name]).unwrap();

        tx.commit().unwrap();

        println!("OK.");
    }

    fn code_exists(&self, code: &str) -> bool {
        let query = "SELECT (
                       SELECT COUNT(*) FROM session WHERE logincode = $1
                     ) + (
                       SELECT COUNT(*) FROM usergroup WHERE groupcode = $1
                     ) AS count";

        let n_rows = self.query_map_one(query, &[&code], |row| row.get::<_, i64>(0) as i32).unwrap().unwrap();

        n_rows > 0
    }

    // fn get_session<T: ToSql>(&self, key: T, keyname: &str) -> Option<SessionUser> {
    fn get_session(&self, key: &str) -> Option<SessionUser> {
        let query = "SELECT id, csrf_token, last_login, last_activity, account_created, username, password,
                            salt, logincode, email, email_unconfirmed, email_confirmationcode, firstname, lastname, street,
                            zip, city, nation, grade, sex, is_admin, is_teacher, managed_by, oauth_provider, oauth_foreign_id
                     FROM session
                     WHERE session_token = $1";
        let session = self.query_map_one(query, &[&key], |row| SessionUser { id: row.get(0),
                                                                             session_token: Some(key.to_string()),
                                                                             csrf_token: row.get(1),
                                                                             last_login: row.get(2),
                                                                             last_activity: row.get(3),
                                                                             account_created: row.get(4),

                                                                             username: row.get(5),
                                                                             password: row.get(6),
                                                                             salt: row.get(7),
                                                                             logincode: row.get(8),
                                                                             email: row.get(9),
                                                                             email_unconfirmed: row.get(10),
                                                                             email_confirmationcode: row.get(11),

                                                                             firstname: row.get(12),
                                                                             lastname: row.get(13),
                                                                             street: row.get(14),
                                                                             zip: row.get(15),
                                                                             city: row.get(16),
                                                                             nation: row.get(17),
                                                                             grade: row.get(18),
                                                                             sex: row.get(19),

                                                                             is_admin: row.get(20),
                                                                             is_teacher: row.get(21),
                                                                             managed_by: row.get(22),

                                                                             oauth_provider: row.get(23),
                                                                             oauth_foreign_id: row.get(24) })
                          .ok()??;

        let session_duration = Duration::hours(12);
        let mimimal_activity_update_duration = Duration::minutes(3);
        let now = time::get_time();

        if let Some(last_activity) = session.last_activity {
            if now < last_activity + session_duration {
                if now > last_activity + mimimal_activity_update_duration {
                    let query = "UPDATE session
                                 SET last_activity = $1
                                 WHERE id = $2";
                    self.execute(query, &[&now, &session.id]).unwrap();
                }
                return Some(session);
            } else {
                // Session timed out
                // Should remove session token from session
                return None;
            }
        }
        // last_activity undefined
        // TODO: What should happen here?
        None
    }
    fn save_session(&self, session: SessionUser) {
        self.execute("UPDATE session
                      SET username = $1,
                          password = $2,
                          salt = $3,
                          logincode = $4,
                          firstname = $5,
                          lastname = $6,
                          street = $7,
                          zip = $8,
                          city = $9,
                          grade = $10,
                          sex = $11,
                          is_admin = $12,
                          is_teacher = $13,
                          managed_by = $14,
                          email = $15,
                          email_unconfirmed = $16
                      WHERE id = $17",
                     &[&session.username,
                       &session.password,
                       &session.salt,
                       &session.logincode,
                       &session.firstname,
                       &session.lastname,
                       &session.street,
                       &session.zip,
                       &session.city,
                       &session.grade,
                       &session.sex,
                       &session.is_admin,
                       &session.is_teacher,
                       &session.managed_by,
                       &session.email,
                       &session.email_unconfirmed,
                       &session.id])
            .unwrap();
    }
    fn new_session(&self, session_token: &str) -> SessionUser {
        let csrf_token = helpers::make_csrf_token();

        let now = time::get_time();
        let query = "INSERT INTO session (session_token, csrf_token, last_activity, account_created, grade, sex,
                                          is_teacher)
                     VALUES ($1, $2, $3, $4, $5, $6, $7)";
        self.execute(query, &[&session_token, &csrf_token, &now, &None::<time::Timespec>, &0, &None::<i32>, &false])
            .unwrap();

        let id = self.get_last_id().expect("Expected to get last row id");

        SessionUser::minimal(id, session_token.to_owned(), csrf_token)
    }
    fn session_set_activity_dates(&self, session_id: i32, account_created: Option<time::Timespec>,
                                  last_login: Option<time::Timespec>, last_activity: Option<time::Timespec>) {
        let query = "UPDATE session
                     SET account_created = $2, last_login = $3, last_activity = $4
                     WHERE id = $1";
        self.execute(query, &[&session_id, &account_created, &last_login, &last_activity]).unwrap();
    }
    fn get_session_or_new(&self, key: &str) -> Result<SessionUser, ()> {
        fn disable_old_session_and_create_new(conn: &Connection, key: &str) -> Result<SessionUser, ()> {
            let query = "UPDATE session
                         SET session_token = $1
                         WHERE session_token = $2";
            // TODO: Should a new session key be generated every time?
            conn.execute(query, &[&Option::<String>::None, &key]).map_err(|_| ())?;
            Ok(conn.new_session(&key))
        }

        if let Some(session) = self.get_session(&key).ensure_alive() {
            Ok(session)
        } else {
            disable_old_session_and_create_new(self, key)
        }
    }

    fn get_user_by_id(&self, user_id: i32) -> Option<SessionUser> {
        let query = "SELECT session_token, csrf_token, last_login, last_activity, account_created, username, password,
                            salt, logincode, email, email_unconfirmed, email_confirmationcode, firstname, lastname,
                            street, zip, city, nation, grade, sex, is_admin, is_teacher, managed_by, oauth_provider,
                            oauth_foreign_id
                     FROM session
                     WHERE id = $1";
        self.query_map_one(query, &[&user_id], |row| SessionUser { id: user_id,
                                                                   session_token: row.get(0),
                                                                   csrf_token: row.get(1),
                                                                   last_login: row.get(2),
                                                                   last_activity: row.get(3),
                                                                   account_created: row.get(4),

                                                                   username: row.get(5),
                                                                   password: row.get(6),
                                                                   salt: row.get(7),
                                                                   logincode: row.get(8),
                                                                   email: row.get(9),
                                                                   email_unconfirmed: row.get(10),
                                                                   email_confirmationcode: row.get(11),

                                                                   firstname: row.get(12),
                                                                   lastname: row.get(13),
                                                                   street: row.get(14),
                                                                   zip: row.get(15),
                                                                   city: row.get(16),
                                                                   nation: row.get(17),
                                                                   grade: row.get(18),
                                                                   sex: row.get(19),

                                                                   is_admin: row.get(20),
                                                                   is_teacher: row.get(21),
                                                                   managed_by: row.get(22),

                                                                   oauth_provider: row.get(23),
                                                                   oauth_foreign_id: row.get(24) })
            .ok()?
    }

    fn get_user_and_group_by_id(&self, user_id: i32) -> Option<(SessionUser, Option<Group>)> {
        let session = self.get_user_by_id(user_id)?;

        let group_id = match session.managed_by {
            Some(id) => id,
            None => return Some((session, None)),
        };

        let query = "SELECT name, groupcode, tag, admin
                     FROM usergroup
                     WHERE id = $1";
        let res = self.query_map_one(query, &[&group_id], |row| Group { id: Some(group_id),
                                                                        name: row.get(0),
                                                                        groupcode: row.get(1),
                                                                        tag: row.get(2),
                                                                        admin: row.get(3),
                                                                        members: Vec::new() })
                      .ok()?;
        match res {
            Some(group) => Some((session, Some(group))),
            _ => Some((session, None)),
        }
    }

    //TODO: use session
    fn login(&self, _session: Option<&str>, username: &str, password: &str) -> Result<String, ()> {
        let query = "SELECT id, password, salt
                     FROM session
                     WHERE username = $1";
        self.query_map_one(query, &[&username], |row| {
                let (id, password_hash, salt): (i32, Option<String>, Option<String>) =
                    (row.get(0), row.get(1), row.get(2));

                //password_hash ist das, was in der Datenbank steht
                if helpers::verify_password(&password,
                                            &salt.ok_or_else(|| println!("salt from database empty"))?,
                                            &password_hash.ok_or_else(|| println!("password from database empty"))?)
                {
                    // TODO: fail more pleasantly
                    // Login okay, update session now!

                    let session_token = helpers::make_session_token();
                    let csrf_token = helpers::make_csrf_token();
                    let now = time::get_time();

                    let query = "UPDATE session
                                 SET session_token = $1, csrf_token = $2, last_login = $3, last_activity = $3
                                 WHERE id = $4";
                    self.execute(query, &[&session_token, &csrf_token, &now, &id]).unwrap();

                    Ok(session_token)
                } else {
                    Err(())
                }
            })
            .map_err(|_| ())?
            .ok_or(())?
    }

    //TODO: use session
    fn login_with_code(&self, _session: Option<&str>, logincode: &str) -> Result<String, ()> {
        if logincode == "" {
            return Err(());
        }

        let query = "SELECT id
                     FROM session
                     WHERE logincode = $1";
        self.query_map_one(query, &[&logincode], |row| {
                // Login okay, update session now!
                let id: i32 = row.get(0);

                let session_token = helpers::make_session_token();
                let csrf_token = helpers::make_csrf_token();
                let now = time::get_time();

                let query = "UPDATE session
                             SET session_token = $1, csrf_token = $2, last_login = $3, last_activity = $3
                             WHERE id = $4";
                self.execute(query, &[&session_token, &csrf_token, &now, &id]).unwrap();

                session_token
            })
            .map_err(|_| ())?
            .ok_or(())
    }

    //TODO: use session
    fn login_foreign(&self, _session: Option<&str>, provider_id: &str, foreign_id: &str,
                     (is_teacher, is_admin, firstname, lastname, sex): (bool, bool, &str, &str, Option<i32>))
                     -> Result<(String, Option<time::Timespec>), ()> {
        let session_token = helpers::make_session_token();
        let csrf_token = helpers::make_csrf_token();
        let now = time::get_time();

        let query = "SELECT id, last_activity
                     FROM session
                     WHERE oauth_foreign_id = $1
                           AND oauth_provider = $2";
        match self.query_map_one(query, &[&foreign_id, &provider_id], |row| -> (i32, time::Timespec) {
                      (row.get(0), row.get(1))
                  }) {
            Ok(Some((id, last_activity))) => {
                let query = "UPDATE session
                             SET session_token = $1, csrf_token = $2, last_login = $3, last_activity = $3,
                                 is_teacher = $4, is_admin = $5,  firstname = $6, lastname = $7, sex = $8
                             WHERE id = $9";
                self.execute(query,
                             &[&session_token,
                               &csrf_token,
                               &now,
                               &is_teacher,
                               &is_admin,
                               &firstname,
                               &lastname,
                               &sex,
                               &id])
                    .unwrap();

                Ok((session_token, Some(last_activity)))
            }
            // Add!
            _ => {
                let query = "INSERT INTO session (session_token, csrf_token, last_login, last_activity,
                                                  account_created, grade, sex, is_teacher, is_admin, oauth_foreign_id,
                                                  oauth_provider, firstname, lastname)
                             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)";
                self.execute(query,
                             &[&session_token,
                               &csrf_token,
                               &now,
                               &now,
                               &now,
                               &(if is_teacher { 255 } else { 0 }),
                               &sex,
                               &is_teacher,
                               &is_admin,
                               &foreign_id,
                               &provider_id,
                               &firstname,
                               &lastname])
                    .unwrap();

                Ok((session_token, None))
            }
        }
    }

    //TODO: use session
    fn create_user_with_groupcode(&self, _session: Option<&str>, groupcode: &str) -> Result<String, ()> {
        if groupcode == "" {
            return Err(());
        }

        let query = "SELECT id
                     FROM usergroup
                     WHERE groupcode = $1";
        let group_id =
            self.query_map_one(query, &[&groupcode], |row| -> i32 { row.get(0) }).map_err(|_| ())?.ok_or(())?;

        // Login okay, create session!
        let session_token = helpers::make_session_token();
        let csrf_token = helpers::make_csrf_token();
        let now = time::get_time();

        // TODO: Refactor this into function
        let mut logincode = String::new();
        for i in 0..10 {
            if i == 9 {
                panic!("ERROR: Too many logincode collisions! Give up ...");
            }
            logincode = helpers::make_logincode();
            if !self.code_exists(&logincode) {
                break;
            }
            println!("WARNING: Logincode collision! Retrying ...");
        }

        let query = "INSERT INTO session (session_token, csrf_token, last_login, last_activity, account_created,
                                          logincode, grade, sex, is_teacher, managed_by)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";
        self.execute(query,
                     &[&session_token, &csrf_token, &now, &now, &now, &logincode, &0, &None::<i32>, &false, &group_id])
            .unwrap();

        Ok(session_token)
    }

    fn update_or_create_group_with_users(&self, mut group: Group) {
        if let Ok(Some(id)) = self.query_map_one("SELECT id FROM usergroup WHERE name = $1 AND tag = $1 AND admin = $2",
                                                 &[&group.name, &group.admin],
                                                 |row| -> i32 { row.get(0) })
        {
            // Set group ID:
            group.set_id(id);
        } else {
            // Generate group ID:
            group.save(self);
        }

        let now = time::get_time();

        for user in group.members {
            if let Ok(Some(id)) =
                self.query_map_one("SELECT id FROM session WHERE firstname = $1 AND lastname = $2 AND managed_by = $3",
                                   &[&user.firstname, &user.lastname, &group.id],
                                   |row| -> i32 { row.get(0) })
            {
                // Update existing user:
                let query = "UPDATE session SET grade = $2, sex = $3 WHERE id = $1";
                self.execute(query, &[&id, &user.grade, &user.sex]).unwrap();
            } else {
                // Generate new user:
                let csrf_token = helpers::make_csrf_token();

                let mut logincode = String::new();
                for i in 0..10 {
                    if i == 9 {
                        panic!("ERROR: Too many logincode collisions! Give up ...");
                    }
                    logincode = helpers::make_logincode();
                    if !self.code_exists(&logincode) {
                        break;
                    }
                    println!("WARNING: Logincode collision! Retrying ...");
                }

                let query =
                    "INSERT INTO session (firstname, lastname, csrf_token, account_created, logincode, grade, sex,
                                          is_teacher, managed_by)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";
                self.execute(query,
                             &[&user.firstname,
                               &user.lastname,
                               &csrf_token,
                               &now,
                               &logincode,
                               &user.grade,
                               &user.sex,
                               &false,
                               &group.id])
                    .unwrap();
            }
        }
    }

    fn logout(&self, session: &str) {
        let query = "UPDATE session
                     SET session_token = NULL
                     WHERE session_token = $1";
        self.execute(query, &[&session]).unwrap();
    }

    fn signup(&self, session_token: &str, username: &str, email: &str, password_hash: String, salt: &str)
              -> SignupResult {
        let mut session_user = self.get_session_or_new(&session_token).unwrap();

        if session_user.is_logged_in() {
            return SignupResult::UserLoggedIn;
        }

        if let Ok(None) = self.query_map_one("SELECT username FROM session WHERE username = $1",
                                             &[&username],
                                             |row| -> Option<String> { row.get(0) })
        {
        } else {
            //This username already exists!
            return SignupResult::UsernameTaken;
        }
        if let Ok(None) = self.query_map_one("SELECT email, email_unconfirmed FROM session WHERE email = $1 OR email_unconfirmed = $1",
                                             &[&email],
                                             |row| -> (Option<String>, Option<String>) { (row.get(0), row.get(1)) })
        {
        } else {
            //This email already exists!
            return SignupResult::EmailTaken;
        }

        session_user.username = Some(username.to_string());
        session_user.email_unconfirmed = Some(email.to_string());
        session_user.password = Some(password_hash);
        session_user.salt = Some(salt.to_string());

        self.save_session(session_user);
        SignupResult::SignedUp
    }

    fn load_submission(&self, session: &SessionUser, task: i32, subtask: Option<&str>) -> Option<Submission> {
        match subtask {
            None => {
                let query = "SELECT id, grade, validated, nonvalidated_grade, value, date, needs_validation
                             FROM submission
                             WHERE task = $1
                             AND session = $2
                             ORDER BY id DESC
                             LIMIT 1";
                self.query_map_one(query, &[&task, &session.id], |row| Submission { id: Some(row.get(0)),
                                                                                    task,
                                                                                    user: session.id,
                                                                                    grade: row.get(1),
                                                                                    validated: row.get(2),
                                                                                    nonvalidated_grade: row.get(3),
                                                                                    subtask_identifier: None,
                                                                                    value: row.get(4),
                                                                                    date: row.get(5),
                                                                                    needs_validation: row.get(6) })
                    .ok()?
            }
            Some(subtask_id) => {
                let query = "SELECT id, grade, validated, nonvalidated_grade, value, date, needs_validation
                             FROM submission
                             WHERE task = $1
                             AND session = $2
                             AND subtask_identifier = $3
                             ORDER BY id DESC
                             LIMIT 1";
                self.query_map_one(query, &[&task, &session.id, &subtask_id], |row| {
                        Submission { id: Some(row.get(0)),
                                     task,
                                     user: session.id,
                                     grade: row.get(1),
                                     validated: row.get(2),
                                     nonvalidated_grade: row.get(3),
                                     subtask_identifier: Some(subtask_id.to_string()),
                                     value: row.get(4),
                                     date: row.get(5),
                                     needs_validation: row.get(6) }
                    })
                    .ok()?
            }
        }
    }
    fn get_all_submissions(&self, session_id: i32, task: i32, subtask: Option<&str>) -> Vec<Submission> {
        match subtask {
            None => {
                let query = "SELECT id, grade, validated, nonvalidated_grade, value, date, needs_validation
                             FROM submission
                             WHERE task = $1
                             AND session = $2
                             ORDER BY date";
                self.query_map_many(query, &[&task, &session_id], |row| Submission { id: Some(row.get(0)),
                                                                                     task,
                                                                                     user: session_id,
                                                                                     grade: row.get(1),
                                                                                     validated: row.get(2),
                                                                                     nonvalidated_grade: row.get(3),
                                                                                     subtask_identifier: None,
                                                                                     value: row.get(4),
                                                                                     date: row.get(5),
                                                                                     needs_validation: row.get(6) })
                    .unwrap()
            }
            _ => unimplemented!(),
        }
    }
    fn submit_submission(&self, mut submission: Submission) {
        submission.save(self);

        let mut grade = self.get_grade_by_submission(submission.id.unwrap());
        if grade.grade.is_none() || submission.grade > grade.grade.unwrap() {
            grade.grade = Some(submission.grade);
            grade.validated = false;
            grade.save(self);
        }
    }
    fn get_grade_by_submission(&self, submission_id: i32) -> Grade {
        let query = "SELECT grade.taskgroup, grade.session, grade.grade, grade.validated
                     FROM grade
                     JOIN task ON grade.taskgroup = task.taskgroup
                     JOIN submission ON task.id = submission.task
                     AND grade.session = submission.session
                     WHERE submission.id = $1";
        self.query_map_one(query, &[&submission_id], |row| Grade { taskgroup: row.get(0),
                                                                   user: row.get(1),
                                                                   grade: row.get(2),
                                                                   validated: row.get(3) })
            .unwrap_or(None)
            .unwrap_or_else(|| {
                let query = "SELECT task.taskgroup, submission.session
                         FROM submission
                         JOIN task ON task.id = submission.task
                         WHERE submission.id = $1";
                self.query_map_one(query, &[&submission_id], |row| Grade { taskgroup: row.get(0),
                                                                           user: row.get(1),
                                                                           grade: None,
                                                                           validated: false })
                    .unwrap()
                    .unwrap() // should this unwrap?
            })
    }

    fn get_contest_groups_grades(&self, session_id: i32, contest_id: i32)
                                 -> (Vec<String>, Vec<(Group, Vec<(UserInfo, Vec<Grade>)>)>) {
        let query = "SELECT id, name
                     FROM taskgroup
                     WHERE contest = $1
                     AND active = $2
                     ORDER BY positionalnumber";
        let tasknames: Vec<(i32, String)> =
            self.query_map_many(query, &[&contest_id, &true], |row| (row.get(0), row.get(1))).unwrap();

        let mut taskindex: ::std::collections::BTreeMap<i32, usize> = ::std::collections::BTreeMap::new();

        let n_tasks = tasknames.len();
        for (index, (i, _)) in tasknames.iter().enumerate() {
            taskindex.insert(*i, index);
        }

        let query = "SELECT grade.taskgroup, grade.session, grade.grade, grade.validated, usergroup.id, usergroup.name,
                            usergroup.groupcode, usergroup.tag, student.id, student.username, student.logincode,
                            student.firstname, student.lastname, student.grade AS sgrade, participation.annotation
                     FROM grade
                     JOIN taskgroup ON grade.taskgroup = taskgroup.id
                     JOIN session AS student ON grade.session = student.id
                     JOIN usergroup ON student.managed_by = usergroup.id
                     JOIN participation ON participation.session = student.id AND participation.contest = $2
                     WHERE usergroup.admin = $1
                     AND taskgroup.contest = $2
                     AND taskgroup.active = $3
                     ORDER BY usergroup.id, sgrade, student.lastname, student.firstname, student.id,
                              taskgroup.positionalnumber";
        let gradeinfo =
            self.query_map_many(query, &[&session_id, &contest_id, &true], |row| {
                    (Grade { taskgroup: row.get(0), user: row.get(1), grade: row.get(2), validated: row.get(3) },
                     Group { id: Some(row.get(4)),
                             name: row.get(5),
                             groupcode: row.get(6),
                             tag: row.get(7),
                             admin: session_id,
                             members: Vec::new() },
                     UserInfo { id: row.get(8),
                                username: row.get(9),
                                logincode: row.get(10),
                                firstname: row.get(11),
                                lastname: row.get(12),
                                grade: row.get(13),
                                annotation: row.get(14) })
                })
                .unwrap();
        let mut gradeinfo_iter = gradeinfo.iter();

        if let Some(t /*Ok((grade, mut group, mut userinfo))*/) = gradeinfo_iter.next() {
            let (grade, mut group, mut userinfo) = t.clone();

            let mut grades: Vec<Grade> = vec![Default::default(); n_tasks];
            let mut users: Vec<(UserInfo, Vec<Grade>)> = Vec::new();
            let mut groups: Vec<(Group, Vec<(UserInfo, Vec<Grade>)>)> = Vec::new();

            let index = grade.taskgroup;
            grades[taskindex[&index]] = grade;

            for ggu in gradeinfo_iter {
                let (g, gr, ui) = ggu;
                if gr.id != group.id {
                    users.push((userinfo, grades));
                    userinfo = ui.clone();
                    grades = vec![Default::default(); n_tasks];

                    groups.push((group, users));
                    group = gr.clone();
                    users = Vec::new();
                } else if ui.id != userinfo.id {
                    users.push((userinfo, grades));
                    userinfo = ui.clone();
                    grades = vec![Default::default(); n_tasks];
                }
                let index = g.taskgroup;
                grades[taskindex[&index]] = *g;
            }
            users.push((userinfo, grades));
            groups.push((group, users));

            (tasknames.iter().map(|(_, name)| name.clone()).collect(), groups)
        } else {
            (Vec::new(), Vec::new()) // should those be default filled?
        }
    }
    fn get_contest_user_grades(&self, session_token: &str, contest_id: i32) -> Vec<Grade> {
        let query = "SELECT id, name
                     FROM taskgroup
                     WHERE contest = $1
                     AND active = $2
                     ORDER BY positionalnumber";
        let tasknames: Vec<(i32, String)> =
            self.query_map_many(query, &[&contest_id, &true], |row| (row.get(0), row.get(1))).unwrap();
        let mut taskindex: ::std::collections::BTreeMap<i32, usize> = ::std::collections::BTreeMap::new();

        let n_tasks = tasknames.len();
        for (index, (i, _)) in tasknames.iter().enumerate() {
            taskindex.insert(*i, index);
        }

        let query = "SELECT grade.taskgroup, grade.session, grade.grade, grade.validated
                     FROM grade
                     JOIN taskgroup ON grade.taskgroup = taskgroup.id
                     JOIN session ON session.id = grade.session
                     WHERE session.session_token = $1
                     AND taskgroup.contest = $2
                     AND taskgroup.active = $3
                     ORDER BY taskgroup.positionalnumber";
        let gradeinfo =
            self.query_map_many(query, &[&session_token, &contest_id, &true], |row| Grade { taskgroup: row.get(0),
                                                                                            user: row.get(1),
                                                                                            grade: row.get(2),
                                                                                            validated: row.get(3) })
                .unwrap();
        let gradeinfo_iter = gradeinfo.iter();

        let mut grades: Vec<Grade> = vec![Default::default(); n_tasks];

        for g in gradeinfo_iter {
            let index = g.taskgroup;
            grades[taskindex[&index]] = *g;
        }

        grades
    }

    fn get_taskgroup_user_grade(&self, session_token: &str, taskgroup_id: i32) -> Grade {
        let query = "SELECT grade.taskgroup, grade.session, grade.grade, grade.validated
                     FROM grade
                     JOIN session ON session.id = grade.session
                     WHERE session.session_token = $1
                     AND grade.taskgroup = $2";
        self.query_map_one(query, &[&session_token, &taskgroup_id], |row| Grade { taskgroup: row.get(0),
                                                                                  user: row.get(1),
                                                                                  grade: row.get(2),
                                                                                  validated: row.get(3) })
            .unwrap_or(None)
            .unwrap_or_default()
    }

    /* Warning: This function makes no use of rusts typeb safety. Handle with care when changeing */
    fn export_contest_results_to_file(&self, contest_id: i32, taskgroups: &[(i32, String)], filename: &str) {
        use std::fs::OpenOptions;
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(filename).unwrap();
        let mut headers = vec!["id",
                               "username",
                               "logincode",
                               "oauth_foreign_id",
                               "oauth_provider",
                               "firstname",
                               "lastname",
                               "grade",
                               "sex",
                               "is_teacher",
                               "group_id",
                               "group_name",
                               "group_tag",
                               "teacher_id",
                               "teacher_firstname",
                               "teacher_lastname",
                               "teacher_oauth_foreign_id",
                               "teacher_oauth_school_id",
                               "teacher_oauth_provider",
                               "contest_id",
                               "start_date"];

        let mut select_part = String::new();
        let mut join_part = String::new();

        let mut join_params = gen_tosql_vector();

        join_params.push(&contest_id);

        for (n, (id, name)) in taskgroups.iter().enumerate() {
            use std::fmt::Write;

            write!(select_part, ",\n g{}.grade ", n).unwrap();
            write!(join_part,
                   "\n LEFT JOIN grade AS g{} ON session.id = g{}.session AND g{}.taskgroup = ${} ",
                   n,
                   n,
                   n,
                   n + 2).unwrap();
            join_params.push(id);
            headers.push(&name);
        }

        let query = format!("SELECT session.id,
                                    session.username,
                                    session.logincode,
                                    session.oauth_foreign_id,
                                    session.oauth_provider,
                                    session.firstname,
                                    session.lastname,
                                    session.grade,
                                    session.sex,
                                    session.is_teacher,
                                    session.managed_by,
                                    usergroup.name,
                                    usergroup.tag,
                                    teacher.id,
                                    teacher.firstname,
                                    teacher.lastname,
                                    teacher.oauth_foreign_id,
                                    teacher.oauth_provider,
                                    participation.contest,
                                    participation.start_date
                                    {}
                             FROM participation
                             JOIN session ON participation.session = session.id
                             {}
                             LEFT JOIN usergroup ON session.managed_by = usergroup.id
                             LEFT JOIN session AS teacher ON usergroup.admin = teacher.id
                             WHERE participation.contest = $1",
                            select_part, join_part);

        use csv::Writer;
        let mut wtr = Writer::from_writer(file);
        wtr.serialize(&headers).unwrap();
        wtr.flush().unwrap();

        let file = wtr.into_inner().unwrap();
        let mut wtr = Writer::from_writer(file);

        self.query_map_many(&query, join_params.as_slice(), |row| {
                let mut points = Vec::new();
                for i in 20..20 + taskgroups.len() {
                    points.push(row.get::<_, Option<i32>>(i));
                }

                let teacher_oauth_and_school_id = row.get::<_, Option<String>>(16);
                let (teacher_oauth_id, teacher_school_id) = if let Some(toasi) = teacher_oauth_and_school_id {
                    let mut v = toasi.split('/');
                    let oid: Option<String> = v.next().map(|s| s.to_owned());
                    let sid: Option<String> = v.next().map(|s| s.to_owned());
                    (oid, sid)
                } else {
                    (None, None)
                };

                // Serialized as several tuples because Serde only supports tuples up to a certain length
                // (16 according to https://docs.serde.rs/serde/trait.Deserialize.html)
                wtr.serialize(((row.get::<_, i32>(0),
                                row.get::<_, Option<String>>(1),
                                row.get::<_, Option<String>>(2),
                                row.get::<_, Option<String>>(3),
                                row.get::<_, Option<String>>(4),
                                row.get::<_, Option<String>>(5),
                                row.get::<_, Option<String>>(6),
                                row.get::<_, i32>(7),
                                row.get::<_, Option<i32>>(8),
                                row.get::<_, bool>(9)),
                               (row.get::<_, Option<i32>>(10),
                                row.get::<_, Option<String>>(11),
                                row.get::<_, Option<String>>(12),
                                row.get::<_, Option<i32>>(13),
                                row.get::<_, Option<String>>(14),
                                row.get::<_, Option<String>>(15),
                                teacher_oauth_id,
                                teacher_school_id,
                                row.get::<_, Option<String>>(17)),
                               row.get::<_, Option<i32>>(18),
                               row.get::<_, Option<time::Timespec>>(19)
                                  .map(|ts| self::time::strftime("%FT%T%z", &time::at(ts)).unwrap()),
                               points))
                   .unwrap();
            })
            .unwrap();
        wtr.flush().unwrap();
    }

    fn insert_contest_annotations(&self, contest_id: i32, annotations: Vec<(i32, Option<String>)>) -> i32 {
        let batch_size = 10;
        let query_batch = "UPDATE participation
                           SET annotation = batchdata.annotation
                           FROM (SELECT $2 AS userid, $3 as annotation
                                 UNION ALL SELECT $4 AS userid, $5 as annotation
                                 UNION ALL SELECT $6 AS userid, $7 as annotation
                                 UNION ALL SELECT $8 AS userid, $9 as annotation
                                 UNION ALL SELECT $10 AS userid, $11 as annotation
                                 UNION ALL SELECT $12 AS userid, $13 as annotation
                                 UNION ALL SELECT $14 AS userid, $15 as annotation
                                 UNION ALL SELECT $16 AS userid, $17 as annotation
                                 UNION ALL SELECT $18 AS userid, $19 as annotation
                                 UNION ALL SELECT $20 AS userid, $21 as annotation
                                ) AS batchdata
                           WHERE session = batchdata.userid
                           AND contest = $1";
        let query_single = "UPDATE participation
                            SET annotation = $3
                            WHERE session = $2
                            AND contest = $1";

        let n_annotations = annotations.len();
        let n_batches = n_annotations / batch_size;
        let n_single = n_annotations % batch_size;

        #[cfg(feature = "debug")]
        println!("Annotations: {}, {} batches a {}, {} single", n_annotations, n_batches, batch_size, n_single);

        let mut rows_modified = 0;

        for batch in 0..n_batches {
            let off = batch * batch_size;
            rows_modified += self.execute(query_batch,
                                          &[&contest_id,
                                            &annotations[off].0,
                                            &annotations[off].1,
                                            &annotations[off + 1].0,
                                            &annotations[off + 1].1,
                                            &annotations[off + 2].0,
                                            &annotations[off + 2].1,
                                            &annotations[off + 3].0,
                                            &annotations[off + 3].1,
                                            &annotations[off + 4].0,
                                            &annotations[off + 4].1,
                                            &annotations[off + 5].0,
                                            &annotations[off + 5].1,
                                            &annotations[off + 6].0,
                                            &annotations[off + 6].1,
                                            &annotations[off + 7].0,
                                            &annotations[off + 7].1,
                                            &annotations[off + 8].0,
                                            &annotations[off + 8].1,
                                            &annotations[off + 9].0,
                                            &annotations[off + 9].1])
                                 .unwrap();
        }

        let off = n_annotations - n_single;
        for single in 0..n_single {
            rows_modified += self.execute(query_single,
                                          &[&contest_id, &annotations[off + single].0, &annotations[off + single].1])
                                 .unwrap();
        }

        rows_modified as i32
    }

    fn get_submission_by_id_complete_shallow_contest(&self, submission_id: i32)
                                                     -> Option<(Submission, Task, Taskgroup, Contest)> {
        let query = "SELECT submission.session, submission.grade, submission.validated, submission.nonvalidated_grade,
                            submission.needs_validation, submission.subtask_identifier, submission.value,
                            submission.date,
                            task.id, task.location, task.language, task.stars,
                            taskgroup.id, taskgroup.name, taskgroup.active, taskgroup.positionalnumber,
                            contest.id, contest.location, contest.filename, contest.name, contest.duration,
                            contest.public, contest.protected
                     FROM submission
                     JOIN task ON task.id = submission.task
                     JOIN taskgroup ON taskgroup.id = task.taskgroup
                     JOIN contest ON contest.id = taskgroup.contest
                     WHERE submission.id = $1";
        self.query_map_one(query, &[&submission_id], |row| {
                (Submission { id: Some(submission_id),
                              user: row.get(0),
                              task: row.get(8),
                              grade: row.get(1),
                              validated: row.get(2),
                              nonvalidated_grade: row.get(3),
                              needs_validation: row.get(4),
                              subtask_identifier: row.get(5),
                              value: row.get(6),
                              date: row.get(7) },
                 Task { id: Some(row.get(8)),
                        taskgroup: row.get(11),
                        location: row.get(9),
                        language: row.get(10),
                        stars: row.get(11) },
                 Taskgroup { id: row.get(12),
                             contest: row.get(16),
                             name: row.get(13),
                             active: row.get(14),
                             positionalnumber: row.get(15),
                             tasks: Vec::new() },
                 Contest { id: row.get(16),
                           location: row.get(17),
                           filename: row.get(18),
                           name: row.get(19),
                           duration: row.get(20),
                           public: row.get(21),
                           start: None,
                           end: None,
                           review_start: None,
                           review_end: None,
                           min_grade: None,
                           max_grade: None,
                           positionalnumber: None,
                           requires_login: None,
                           requires_contest: None,
                           protected: row.get(22),
                           secret: None,
                           message: None,
                           image: None,
                           language: None,
                           category: None,
                           standalone_task: None,
                           tags: Vec::new(),
                           taskgroups: Vec::new() })
            })
            .unwrap()
    }

    fn get_contest_list(&self) -> Vec<Contest> {
        let query = "SELECT id, location, filename, name, duration, public, start_date, end_date, review_start_date,
                            review_end_date, min_grade, max_grade, positionalnumber, protected, requires_login,
                            requires_contest, secret, message, image, language, category, standalone_task, tags
                     FROM contest
                     LEFT JOIN contest_tags USING (id)
                     ORDER BY positionalnumber DESC";
        self.query_map_many(query, &[], |row| Contest { id: Some(row.get(0)),
                                                        location: row.get(1),
                                                        filename: row.get(2),
                                                        name: row.get(3),
                                                        duration: row.get(4),
                                                        public: row.get(5),
                                                        start: row.get(6),
                                                        end: row.get(7),
                                                        review_start: row.get(8),
                                                        review_end: row.get(9),
                                                        min_grade: row.get(10),
                                                        max_grade: row.get(11),
                                                        positionalnumber: row.get(12),
                                                        protected: row.get(13),
                                                        requires_login: row.get(14),
                                                        requires_contest: row.get(15),
                                                        secret: row.get(16),
                                                        message: row.get(17),
                                                        image: row.get(18),
                                                        language: row.get(19),
                                                        category: row.get(20),
                                                        standalone_task: row.get(21),
                                                        tags: row.get::<_, Option<String>>(22)
                                                                 .map(|tags| {
                                                                     tags.split(',').map(|tag| tag.to_owned()).collect()
                                                                 })
                                                                 .unwrap_or_else(Vec::new),
                                                        taskgroups: Vec::new() })
            .unwrap()
    }

    fn get_contest_by_id(&self, contest_id: i32) -> Option<Contest> {
        let query = "SELECT location, filename, name, duration, public, start_date, end_date, review_start_date,
                            review_end_date, min_grade, max_grade, protected, requires_login, requires_contest, secret,
                            message, image, language, category, standalone_task
                     FROM contest
                     WHERE id = $1";
        self.query_map_one(query, &[&contest_id], |row| Contest { id: Some(contest_id),
                                                                  location: row.get(0),
                                                                  filename: row.get(1),
                                                                  name: row.get(2),
                                                                  duration: row.get(3),
                                                                  public: row.get(4),
                                                                  start: row.get(5),
                                                                  end: row.get(6),
                                                                  review_start: row.get(7),
                                                                  review_end: row.get(8),
                                                                  min_grade: row.get(9),
                                                                  max_grade: row.get(10),
                                                                  positionalnumber: None,
                                                                  protected: row.get(11),
                                                                  requires_login: row.get(12),
                                                                  requires_contest: row.get(13),
                                                                  secret: row.get(14),
                                                                  message: row.get(15),
                                                                  image: row.get(16),
                                                                  language: row.get(17),
                                                                  category: row.get(18),
                                                                  standalone_task: row.get(19),
                                                                  tags: Vec::new(),
                                                                  taskgroups: Vec::new() })
            .unwrap()
    }

    fn get_contest_by_id_complete(&self, contest_id: i32) -> Option<Contest> {
        let query = "SELECT contest.location, contest.filename, contest.name, contest.duration, contest.public,
                            contest.start_date, contest.end_date, contest.review_start_date, contest.review_end_date,
                            contest.min_grade, contest.max_grade, contest.protected, contest.requires_login,
                            contest.requires_contest, contest.secret, contest.message, contest.image, contest.language,
                            contest.category, contest.standalone_task,
                            taskgroup.id, taskgroup.name,
                            task.id, task.location, task.language, task.stars
                     FROM contest
                     JOIN taskgroup ON contest.id = taskgroup.contest
                     JOIN task ON taskgroup.id = task.taskgroup
                     WHERE contest.id = $1
                     AND taskgroup.active = $2
                     ORDER BY taskgroup.positionalnumber";
        let taskgroupcontest = self.query_map_many(query, &[&contest_id, &true], |row| {
                                       (Contest { id: Some(contest_id),
                                                  location: row.get(0),
                                                  filename: row.get(1),
                                                  name: row.get(2),
                                                  duration: row.get(3),
                                                  public: row.get(4),
                                                  start: row.get(5),
                                                  end: row.get(6),
                                                  review_start: row.get(7),
                                                  review_end: row.get(8),
                                                  min_grade: row.get(9),
                                                  max_grade: row.get(10),
                                                  positionalnumber: None,
                                                  protected: row.get(11),
                                                  requires_login: row.get(12),
                                                  requires_contest: row.get(13),
                                                  secret: row.get(14),
                                                  message: row.get(15),
                                                  image: row.get(16),
                                                  language: row.get(17),
                                                  category: row.get(18),
                                                  standalone_task: row.get(19),
                                                  tags: Vec::new(),
                                                  taskgroups: Vec::new() },
                                        Taskgroup { id: Some(row.get(20)),
                                                    contest: contest_id,
                                                    name: row.get(21),
                                                    active: true,
                                                    positionalnumber: None,
                                                    tasks: Vec::new() },
                                        Task { id: Some(row.get(22)),
                                               taskgroup: row.get(20),
                                               location: row.get(23),
                                               language: row.get(24),
                                               stars: row.get(25) })
                                   })
                                   .unwrap();
        let mut taskgroupcontest_iter = taskgroupcontest.into_iter();

        if let Some((mut contest, mut taskgroup, task)) = taskgroupcontest_iter.next() {
            taskgroup.tasks.push(task);
            for tgc in taskgroupcontest_iter {
                let (_, tg, t) = tgc;
                if tg.id != taskgroup.id {
                    contest.taskgroups.push(taskgroup);
                    taskgroup = tg;
                }
                taskgroup.tasks.push(t);
            }
            contest.taskgroups.push(taskgroup);
            Some(contest)
        } else {
            // If the contest has no tasks, we fall back to the function, that does not try to gather the task
            // information
            self.get_contest_by_id(contest_id)
        }
    }

    fn get_contest_by_id_partial(&self, contest_id: i32) -> Option<Contest> {
        let query = "SELECT contest.location, contest.filename, contest.name, contest.duration, contest.public,
                            contest.start_date, contest.end_date, contest.review_start_date, contest.review_end_date,
                            contest.min_grade, contest.max_grade, contest.protected, contest.requires_login,
                            contest.requires_contest, contest.secret, contest.message, contest.image, contest.language,
                            contest.category, contest.standalone_task,
                            taskgroup.id, taskgroup.name
                     FROM contest
                     JOIN taskgroup ON contest.id = taskgroup.contest
                     WHERE contest.id = $1
                     AND taskgroup.active = $2";
        let taskgroupcontest = self.query_map_many(query, &[&contest_id, &true], |row| {
                                       (Contest { id: Some(contest_id),
                                                  location: row.get(0),
                                                  filename: row.get(1),
                                                  name: row.get(2),
                                                  duration: row.get(3),
                                                  public: row.get(4),
                                                  start: row.get(5),
                                                  end: row.get(6),
                                                  review_start: row.get(7),
                                                  review_end: row.get(8),
                                                  min_grade: row.get(9),
                                                  max_grade: row.get(10),
                                                  positionalnumber: None,
                                                  protected: row.get(11),
                                                  requires_login: row.get(12),
                                                  requires_contest: row.get(13),
                                                  secret: row.get(14),
                                                  message: row.get(15),
                                                  image: row.get(16),
                                                  language: row.get(17),
                                                  category: row.get(18),
                                                  standalone_task: row.get(19),
                                                  tags: Vec::new(),
                                                  taskgroups: Vec::new() },
                                        Taskgroup { id: Some(row.get(20)),
                                                    contest: contest_id,
                                                    name: row.get(21),
                                                    active: true,
                                                    positionalnumber: None,
                                                    tasks: Vec::new() })
                                   })
                                   .unwrap();
        let mut taskgroupcontest_iter = taskgroupcontest.into_iter();

        if let Some((mut contest, taskgroup)) = taskgroupcontest_iter.next() {
            contest.taskgroups.push(taskgroup);
            for tgc in taskgroupcontest_iter {
                let (_, tg) = tgc;
                contest.taskgroups.push(tg);
            }
            Some(contest)
        } else {
            // If the contest has no tasks, we fall back to the function, that does not try to gather the task
            // information
            self.get_contest_by_id(contest_id)
        }
    }

    fn get_participation(&self, session_id: i32, contest_id: i32) -> Option<Participation> {
        let query = "SELECT start_date
                     FROM participation
                     WHERE session = $1
                     AND contest = $2";
        self.query_map_one(query, &[&session_id, &contest_id], |row| Participation { contest: contest_id,
                                                                                     user: session_id,
                                                                                     start: row.get(0) })
            .ok()?
    }

    fn get_own_participation(&self, session: &str, contest_id: i32) -> Option<Participation> {
        let query = "SELECT session, start_date
                     FROM participation
                     JOIN session ON session.id = session
                     WHERE session.session_token = $1
                     AND contest = $2";
        self.query_map_one(query, &[&session, &contest_id], |row| Participation { contest: contest_id,
                                                                                  user: row.get(0),
                                                                                  start: row.get(1) })
            .ok()?
    }

    fn get_all_participations_complete(&self, session_id: i32) -> Vec<(Participation, Contest)> {
        let query = "SELECT participation.start_date, contest.id, location, filename, name, duration, public,
                            contest.start_date, end_date, review_start_date, review_end_date, min_grade, max_grade,
                            protected, requires_login, requires_contest, secret, message, category
                     FROM participation
                     JOIN contest ON participation.contest = contest.id
                     WHERE participation.session = $1 AND (standalone_task IS NULL OR standalone_task = FALSE)";
        self.query_map_many(query, &[&session_id], |row| {
                (Participation { contest: row.get(1), user: session_id, start: row.get(0) },
                 Contest { id: Some(row.get(1)),
                           location: row.get(2),
                           filename: row.get(3),
                           name: row.get(4),
                           duration: row.get(5),
                           public: row.get(6),
                           start: row.get(7),
                           end: row.get(8),
                           review_start: row.get(9),
                           review_end: row.get(10),
                           min_grade: row.get(11),
                           max_grade: row.get(12),
                           positionalnumber: None,
                           protected: row.get(13),
                           requires_login: row.get(14),
                           requires_contest: row.get(15),
                           secret: row.get(16),
                           message: row.get(17),
                           image: None,
                           language: None,
                           category: row.get(18),
                           standalone_task: None,
                           tags: Vec::new(),
                           taskgroups: Vec::new() })
            })
            .unwrap()
    }

    fn count_all_stars(&self, session_id: i32) -> i32 {
        let query = "SELECT COALESCE(SUM(grade.grade), 0) AS stars
                     FROM grade
                     WHERE session = $1";
        self.query_map_one(query, &[&session_id], |row| -> i64 { row.get(0) }).unwrap().unwrap() as i32
    }

    fn has_participation_by_contest_file(&self, session_id: i32, location: &str, filename: &str) -> bool {
        let query = "SELECT participation.contest
                     FROM participation
                     JOIN contest ON participation.contest = contest.id
                     WHERE participation.session = $1
                     AND contest.location = $2
                     AND contest.filename = $3";
        self.exists(query, &[&session_id, &location, &filename])
    }

    fn new_participation(&self, session: &str, contest_id: i32) -> Result<Participation, ()> {
        let query = "SELECT session, start_date
                     FROM participation
                     JOIN session ON session.id = session
                     WHERE session.session_token = $1
                     AND contest = $2";
        match self.query_map_one(query, &[&session, &contest_id], |_| {}).map_err(|_| ())? {
            Some(()) => Err(()),
            None => {
                let now = time::get_time();
                self.execute(
                             "INSERT INTO participation (contest, session, start_date)
                     SELECT $1, id, $2 FROM session WHERE session_token = $3",
                             &[&contest_id, &now, &session],
                )
                    .unwrap();

                Ok(self.get_own_participation(session, contest_id).unwrap()) // TODO: This errors if not logged in …
            }
        }
    }
    fn get_task_by_id(&self, task_id: i32) -> Option<Task> {
        let query = "SELECT location, language, stars, taskgroup
                     FROM task
                     WHERE id = $1";
        self.query_map_one(query, &[&task_id], |row| Task { id: Some(task_id),
                                                            taskgroup: row.get(3),
                                                            location: row.get(0),
                                                            language: row.get(1),
                                                            stars: row.get(2) })
            .unwrap()
    }
    fn get_task_by_id_complete(&self, task_id: i32) -> Option<(Task, Taskgroup, Contest)> {
        let query = "SELECT task.location, task.language, task.stars,
                            taskgroup.id, taskgroup.name, taskgroup.active,
                            contest.id, contest.location, contest.filename, contest.name, contest.duration,
                            contest.public, contest.start_date, contest.end_date, contest.review_start_date,
                            contest.review_end_date, contest.min_grade, contest.max_grade, contest.protected,
                            contest.requires_login, contest.requires_contest, contest.secret, contest.message,
                            contest.category, contest.standalone_task
                     FROM contest
                     JOIN taskgroup ON taskgroup.contest = contest.id
                     JOIN task ON task.taskgroup = taskgroup.id
                     WHERE task.id = $1";
        self.query_map_one(query, &[&task_id], |row| {
                (Task { id: Some(task_id),
                        taskgroup: row.get(3),
                        location: row.get(0),
                        language: row.get(1),
                        stars: row.get(2) },
                 Taskgroup { id: Some(row.get(3)),
                             contest: row.get(6),
                             name: row.get(4),
                             active: row.get(5),
                             positionalnumber: None,
                             tasks: Vec::new() },
                 Contest { id: Some(row.get(6)),
                           location: row.get(7),
                           filename: row.get(8),
                           name: row.get(9),
                           duration: row.get(10),
                           public: row.get(11),
                           start: row.get(12),
                           end: row.get(13),
                           review_start: row.get(14),
                           review_end: row.get(15),
                           min_grade: row.get(16),
                           max_grade: row.get(17),
                           positionalnumber: None,
                           protected: row.get(18),
                           requires_login: row.get(19),
                           requires_contest: row.get(20),
                           secret: row.get(21),
                           message: row.get(22),
                           image: None,
                           language: None,
                           category: row.get(23),
                           standalone_task: row.get(24),
                           tags: Vec::new(),
                           taskgroups: Vec::new() })
            })
            .unwrap()
    }

    fn get_submission_to_validate(&self, tasklocation: &str, subtask: Option<&str>) -> i32 {
        match subtask {
            Some(st) => {
                let query = "SELECT id
                             FROM submission
                             JOIN task ON submission.task = task.id
                             WHERE task.location = $1
                             AND subtask_identifier = $2
                             AND needs_validation = 1
                             LIMIT 1";
                self.query_map_one(query, &[&tasklocation, &st], |row| row.get(0)).unwrap().unwrap()
            }
            None => {
                let query = "SELECT id
                             FROM submission
                             JOIN task ON submission.task = task.id
                             WHERE task.location = $1
                             AND needs_validation = 1
                             LIMIT 1";
                self.query_map_one(query, &[&tasklocation], |row| row.get(0)).unwrap().unwrap()
            }
        }
    }

    fn find_next_submission_to_validate(&self, userid: i32, taskgroupid: i32) {
        let query = "SELECT id, validated
                     FROM submission
                     JOIN task ON submission.task = task.id
                     WHERE task.taskgroup = $1
                     AND submission.session = $2
                     ORDER BY value DESC id DESC
                     LIMIT 1";
        let (id, validated): (i32, bool) =
            self.query_map_one(query, &[&taskgroupid, &userid], |row| (row.get(0), row.get(1))).unwrap().unwrap();
        if !validated {
            let query = "UPDATE submission
                         SET needs_validation = 1
                         WHERE id = $1";
            self.execute(query, &[&id]).unwrap();
        }
    }

    fn add_group(&self, group: &mut Group) { group.save(self); }

    fn get_groups(&self, session_id: i32) -> Vec<Group> {
        let query = "SELECT id, name, groupcode, tag
                     FROM usergroup
                     WHERE admin = $1";
        self.query_map_many(query, &[&session_id], |row| Group { id: Some(row.get(0)),
                                                                 name: row.get(1),
                                                                 groupcode: row.get(2),
                                                                 tag: row.get(3),
                                                                 admin: session_id,
                                                                 members: Vec::new() })
            .unwrap()
    }
    fn get_groups_complete(&self, _session_id: i32) -> Vec<Group> {
        unimplemented!();
    }
    fn get_group(&self, group_id: i32) -> Option<Group> {
        let query = "SELECT name, groupcode, tag, admin
                     FROM usergroup
                     WHERE id  = $1";
        self.query_map_one(query, &[&group_id], |row| Group { id: Some(group_id),
                                                              name: row.get(0),
                                                              groupcode: row.get(1),
                                                              tag: row.get(2),
                                                              admin: row.get(3),
                                                              members: Vec::new() })
            .unwrap()
    }
    fn group_has_protected_participations(&self, group_id: i32) -> bool {
        let query = "SELECT EXISTS(
                         SELECT session.id
                         FROM session
                         JOIN participation ON participation.session = session.id
                         JOIN contest ON contest.id = participation.contest
                         WHERE managed_by = $1
                         AND contest.protected = $2
                     )";
        self.query_map_one(query, &[&group_id, &true], |row| row.get(0)).unwrap().unwrap()
    }
    fn get_group_complete(&self, group_id: i32) -> Option<Group> {
        let query = "SELECT name, groupcode, tag, admin
                     FROM usergroup
                     WHERE id  = $1";
        let mut group = self.query_map_one(query, &[&group_id], |row| Group { id: Some(group_id),
                                                                              name: row.get(0),
                                                                              groupcode: row.get(1),
                                                                              tag: row.get(2),
                                                                              admin: row.get(3),
                                                                              members: Vec::new() })
                            .unwrap()
                            .unwrap(); // TODO handle error

        let query = "SELECT id, session_token, csrf_token, last_login, last_activity, account_created, username,
                            password, logincode, email, email_unconfirmed, email_confirmationcode, firstname, lastname,
                            street, zip, city, nation, grade, sex, is_admin, is_teacher, oauth_provider,
                            oauth_foreign_id, salt
                     FROM session
                     WHERE managed_by = $1
                     ORDER BY id";
        group.members = self.query_map_many(query, &[&group_id], |row| SessionUser { id: row.get(0),
                                                                                     session_token: row.get(1),
                                                                                     csrf_token: row.get(2),
                                                                                     last_login: row.get(3),
                                                                                     last_activity: row.get(4),
                                                                                     account_created: row.get(5),

                                                                                     username: row.get(6),
                                                                                     password: row.get(7),
                                                                                     salt: row.get(22),
                                                                                     logincode: row.get(8),
                                                                                     email: row.get(9),
                                                                                     email_unconfirmed: row.get(10),
                                                                                     email_confirmationcode:
                                                                                         row.get(11),

                                                                                     firstname: row.get(12),
                                                                                     lastname: row.get(13),
                                                                                     street: row.get(14),
                                                                                     zip: row.get(15),
                                                                                     city: row.get(16),
                                                                                     nation: row.get(17),
                                                                                     grade: row.get(18),
                                                                                     sex: row.get(19),

                                                                                     is_admin: row.get(20),
                                                                                     is_teacher: row.get(21),
                                                                                     managed_by: Some(group_id),

                                                                                     oauth_provider: row.get(22),
                                                                                     oauth_foreign_id: row.get(23) })
                            .unwrap();
        Some(group)
    }

    fn delete_user(&self, user_id: i32) {
        let query = "DELETE FROM session
                     WHERE id = $1";
        self.execute(query, &[&user_id]).unwrap();
    }
    fn delete_all_users_for_group(&self, group_id: i32) {
        let query = "DELETE FROM session
                     WHERE managed_by = $1";
        self.execute(query, &[&group_id]).unwrap();
    }
    fn delete_group(&self, group_id: i32) {
        let query = "DELETE FROM usergroup
                     WHERE id = $1";
        self.execute(query, &[&group_id]).unwrap();
    }
    fn delete_participation(&self, user_id: i32, contest_id: i32) {
        let query = "DELETE FROM submission
                     WHERE id IN (
                         SELECT submission.id FROM submission
                         JOIN task ON submission.task = task.id
                         JOIN taskgroup ON task.taskgroup = taskgroup.id
                         WHERE taskgroup.contest = $1
                         AND submission.session = $2
                     )";
        self.execute(query, &[&contest_id, &user_id]).unwrap();

        let query = "DELETE FROM grade
                     WHERE taskgroup IN (
                         SELECT id FROM taskgroup
                         WHERE taskgroup.contest = $1
                     )
                     AND session = $2";
        self.execute(query, &[&contest_id, &user_id]).unwrap();

        let query = "DELETE FROM participation
                     WHERE contest = $1
                     AND session = $2";
        self.execute(query, &[&contest_id, &user_id]).unwrap();
    }

    fn get_search_users(
        &self,
        (s_id, s_firstname, s_lastname, s_logincode, s_groupcode, s_pms_id): (Option<i32>,
         Option<String>,
         Option<String>,
         Option<String>,
         Option<String>,
         Option<String>))
        -> Result<Vec<(i32, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)>,
                  Vec<(i32, String, String, String)>> {
        if let Some(id) = s_id {
            let query = "SELECT id, firstname, lastname, logincode, oauth_foreign_id, oauth_provider
                         FROM session
                         WHERE id = $1
                         LIMIT 201";
            Ok(self.query_map_many(query, &[&id], |row| {
                       (row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5))
                   })
                   .unwrap())
        } else if let Some(logincode) = s_logincode {
            let query = "SELECT id, firstname, lastname, logincode, oauth_foreign_id, oauth_provider
                         FROM session
                         WHERE logincode = $1
                         LIMIT 201";
            Ok(self.query_map_many(query, &[&logincode], |row| {
                       (row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5))
                   })
                   .unwrap())
        } else if let Some(groupcode) = s_groupcode {
            let query = "SELECT id, name, tag
                         FROM usergroup
                         WHERE groupcode = $1
                         LIMIT 201";
            Err(self.query_map_many(query, &[&groupcode], |row| {
                        (row.get(0), row.get(1), row.get(2), groupcode.clone())
                    })
                    .unwrap())
        } else if let Some(pms_id) = s_pms_id {
            let query = "SELECT id, firstname, lastname, logincode, oauth_foreign_id, oauth_provider
                         FROM session
                         WHERE oauth_foreign_id = $1
			 OR oauth_foreign_id LIKE $2
                         LIMIT 201";
            Ok(self.query_map_many(query, &[&pms_id, &format!("{}/%", pms_id)], |row| {
                       (row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5))
                   })
                   .unwrap())
        } else if let (Some(firstname), Some(lastname)) = (s_firstname, s_lastname) {
            let query = "SELECT id, firstname, lastname, logincode, oauth_foreign_id, oauth_provider
                         FROM session
                         WHERE firstname ILIKE $1
                         AND lastname ILIKE $2
                         ORDER BY id DESC
                         LIMIT 201";
            Ok(self.query_map_many(query, &[&format!("%{}%", firstname), &format!("%{}%", lastname)], |row| {
                       (row.get(0), row.get(1), row.get(2), row.get(3), row.get(4), row.get(5))
                   })
                   .unwrap())
        } else {
            Ok(Vec::new())
        }
    }

    // TODO, should those unwraps be handled?
    fn remove_old_users_and_groups(&self, maxstudentage: time::Timespec, maxteacherage: Option<time::Timespec>,
                                   maxage: Option<time::Timespec>)
                                   -> Result<(i32, i32, i32, i32), ()> {
        // Get list of all groups where students will be removed
        let query = "SELECT managed_by
                     FROM session
                     WHERE username IS NULL AND password IS NULL AND oauth_foreign_id IS NULL AND oauth_provider IS NULL AND managed_by IS NOT NULL
                     AND ((last_login < $1 AND last_activity < $1)
                          OR (last_login < $1 AND last_activity IS NULL)
                          OR (last_login IS NULL AND last_activity < $1)
                          OR (last_login IS NULL AND last_activity IS NULL AND account_created < $1))";
        let mut groups: Vec<i32> = self.query_map_many(query, &[&maxstudentage], |row| row.get(0)).unwrap();

        // Remove students
        let query = "DELETE
                     FROM session
                     WHERE username IS NULL AND password IS NULL AND oauth_foreign_id IS NULL AND oauth_provider IS NULL
                     AND ((last_login < $1 AND last_activity < $1)
                          OR (last_login < $1 AND last_activity IS NULL)
                          OR (last_login IS NULL AND last_activity < $1)
                          OR (last_login IS NULL AND last_activity IS NULL AND account_created < $1))";
        self.execute(query, &[&maxstudentage]).unwrap();

        // Bookkeeping
        let n_users = groups.len() as i32;
        let mut n_groups: i32 = 0;
        let mut n_teachers: i32 = 0;
        let mut n_other: i32 = 0;

        // Get list of groups, where users have been removed from
        groups.sort_unstable();
        groups.dedup();

        // Delete all groups that became empty by removing students
        let query = "SELECT count(*)
                     FROM session
                     WHERE managed_by = $1;";
        for group in groups {
            let groupsize: i64 = self.query_map_one(query, &[&group], |row| row.get(0)).unwrap().unwrap();

            if groupsize == 0 {
                let query = "DELETE
                             FROM usergroup
                             WHERE id = $1";
                self.execute(query, &[&group]).unwrap();

                n_groups += 1;
            }
        }

        // Delete all other empty groups that are too old but never had any users
        let query = "SELECT id
                     FROM usergroup
                     WHERE group_created < $1";
        let groups: Vec<i32> = self.query_map_many(query, &[&maxstudentage], |row| row.get(0)).unwrap();
        let query = "SELECT count(*)
                     FROM session
                     WHERE managed_by = $1;";
        for group in groups {
            let groupsize: i64 = self.query_map_one(query, &[&group], |row| row.get(0)).unwrap().unwrap();

            if groupsize == 0 {
                let query = "DELETE
                             FROM usergroup
                             WHERE id = $1";
                self.execute(query, &[&group]).unwrap();

                n_groups += 1;
            }
        }

        // Remove teachers
        let query = "SELECT id
                     FROM session
                     WHERE is_teacher = $1
                     AND ((last_login < $2 AND last_activity < $2)
                          OR (last_login < $2 AND last_activity IS NULL)
                          OR (last_login IS NULL AND last_activity < $2)
                          OR (last_login IS NULL AND last_activity IS NULL AND account_created < $2))";
        if let Some(maxteacherage) = maxteacherage {
            let teachers: Vec<i32> = self.query_map_many(query, &[&true, &maxteacherage], |row| row.get(1)).unwrap();

            // Only remove if no groups are remaining
            let query = "SELECT count(*)
                         FROM usergroup
                         WHERE admin = $1;";
            for teacher in teachers {
                let groupcount: i64 = self.query_map_one(query, &[&teacher], |row| row.get(0)).unwrap().unwrap();

                if groupcount == 0 {
                    let query = "DELETE
                                 FROM session
                                 WHERE id = $1";
                    self.execute(query, &[&teacher]).unwrap();

                    n_teachers += 1;
                }
            }
        }

        // Remove other users
        if let Some(maxage) = maxage {
            let query = "SELECT count(*)
                         FROM session
                         WHERE ((last_login < $1 AND last_activity < $1)
                                OR (last_login < $1 AND last_activity IS NULL)
                                OR (last_login IS NULL AND last_activity < $1)
                                OR (last_login IS NULL AND last_activity IS NULL AND account_created < $1))";
            n_other = self.query_map_one(query, &[&maxage], |row| row.get::<_, i64>(0) as i32).unwrap().unwrap();

            let query = "DELETE
                         FROM session
                         WHERE ((last_login < $1 AND last_activity < $1)
                                OR (last_login < $1 AND last_activity IS NULL)
                                OR (last_login IS NULL AND last_activity < $1)
                                OR (last_login IS NULL AND last_activity IS NULL AND account_created < $1))";
            self.execute(query, &[&maxage]).unwrap();
        }

        Ok((n_users, n_groups, n_teachers, n_other))
    }

    fn remove_temporary_sessions(&self, maxage: time::Timespec) -> Result<(i32, String), ()> {
        // WARNING: This function could possibly be dangerous if the login possibilities change in a way
        // that not every possibility is covered her …
        // TODO: How can we make sure, this function is always safe, even in cases of changes elsewhere?

        let now = time::get_time();
        let cache_key = "last_cleanup";

        let query = "SELECT value, date
                     FROM string_cache
                     WHERE key = $1";

        let cache =
            self.query_map_one(query, &[&cache_key], |row| -> (String, time::Timespec) { (row.get(0), row.get(1)) })
                .unwrap();
        if let Some((ref cached_value, cache_date)) = cache {
            // Cache invalidates once per hour
            if cache_date.sec / (60 * 60) >= now.sec / (60 * 60) {
                return Ok((cached_value.parse().unwrap_or(-1), self::time::strftime("%e. %b %Y, %H:%M", &time::at(cache_date)).unwrap_or("could not format".to_string())));
            }
        }

        let query = "SELECT count(*)
                     FROM session
                     WHERE (last_activity < $1 OR last_activity IS NULL)
                     AND logincode IS NULL
                     AND password IS NULL
                     AND oauth_foreign_id IS NULL";
        let n_session = self.query_map_one(query, &[&maxage], |row| row.get::<_, i64>(0) as i32).unwrap().unwrap();

        let query = "DELETE
                     FROM session
                     WHERE (last_activity < $1 OR last_activity IS NULL)
                     AND logincode IS NULL
                     AND password IS NULL
                     AND oauth_foreign_id IS NULL";
        self.execute(query, &[&maxage]).unwrap();

        let result = format!("{}", n_session);
        let query = if cache.is_some() {
            "UPDATE string_cache
             SET value = $2, date = $3
             WHERE key = $1"
        } else {
            "INSERT INTO string_cache (key, value, date)
             VALUES ($1, $2, $3)"
        };
        self.execute(query, &[&cache_key, &result, &now]).unwrap();

        Ok((n_session, self::time::strftime("%e. %b %Y, %H:%M", &time::at(cache.unwrap_or(("".to_string(), now)).1)).unwrap_or("could not format".to_string())))
    }

    fn get_debug_information(&self) -> String {
        let now = time::get_time();
        let cache_key = "dbstatus";

        let query = "SELECT value, date
                     FROM string_cache
                     WHERE key = $1";

        let db_has_value = if let Some((cached_value, cache_date))//: Option<>
            = self.query_map_one(query, &[&cache_key], |row| -> (String, time::Timespec) {(row.get(0), row.get(1))}).unwrap() {
                // Cache invalidates once per minute
                if cache_date.sec / 60 >= now.sec / 60 {
                    return cached_value;
                }
                true
            } else {
                false
            };

        let duration = Duration::minutes(60);
        let then = now - duration;

        // Zeit: 26,800 ms
        let query = "SELECT count(*)
                     FROM session
                     WHERE last_activity > $1;";
        let n_asession: i64 = self.query_map_one(query, &[&then], |row| row.get(0)).unwrap().unwrap();

        // Zeit: 29,514 ms
        let query = "SELECT count(*)
                     FROM participation
                     WHERE start_date > $1;";
        let n_apart: i64 = self.query_map_one(query, &[&then], |row| row.get(0)).unwrap().unwrap();

        // Zeit: 11,011 ms
        let query = "SELECT count(*)
                     FROM session;";
        let n_session: i64 = self.query_map_one(query, &[], |row| row.get(0)).unwrap().unwrap();

        // Zeit: 26,959 ms
        let query = "SELECT count(*)
                     FROM session
                     WHERE oauth_foreign_id IS NOT NULL OR logincode IS NOT NULL;";
        let n_user: i64 = self.query_map_one(query, &[], |row| row.get(0)).unwrap().unwrap();

        // Zeit: 25,129 ms
        let query = "SELECT count(*)
                     FROM session
                     WHERE oauth_foreign_id IS NOT NULL;";
        let n_pmsuser: i64 = self.query_map_one(query, &[], |row| row.get(0)).unwrap().unwrap();

        // Zeit: 0,264 ms
        let query = "SELECT count(*)
                     FROM session
                     WHERE is_teacher = $1;";
        let n_teacher: i64 = self.query_map_one(query, &[&true], |row| row.get(0)).unwrap().unwrap();

        // Zeit: 10,519 ms
        let query = "SELECT count(*)
                     FROM participation;";
        let n_part: i64 = self.query_map_one(query, &[], |row| row.get(0)).unwrap().unwrap();

        // Zeit: 1205,003 ms (00:01,205)
        // Currently disable to reduce load during contest
        /*let query = "SELECT count(*)
        FROM submission;";*/
        let n_sub: i64 = 0; /*self.query_map_one(query, &[], |row| row.get(0)).unwrap().unwrap();*/

        // Zeit: 19,947 ms
        let query = "SELECT contest, count(*)
                     FROM participation
                     GROUP BY contest
                     ORDER BY contest DESC;";
        let n_participations_by_id: Vec<(i32, i64)> =
            self.query_map_many(query, &[], |row| (row.get(0), row.get(1))).unwrap();

        let result = format!(
                             "{{
  \"timestamp\": {},
  \"active_sessions\": {},
  \"active_participations\": {},
  \"sessions\": {},
  \"users\": {},
  \"pms_users\": {},
  \"teachers\": {},
  \"participations\": {},
  \"submissions\": {},
  \"participations_by_contest_id\": {{
    {}
  }}
}}
",
                             now.sec,
                             n_asession,
                             n_apart,
                             n_session,
                             n_user,
                             n_pmsuser,
                             n_teacher,
                             n_part,
                             n_sub,
                             n_participations_by_id.iter()
                                                   .map(|(x, y)| -> String { format!("\"{}\": {}", x, y) })
                                                   .collect::<Vec<String>>()
                                                   .join(",\n    ")
        );

        let query = if db_has_value {
            "UPDATE string_cache
             SET value = $2, date = $3
             WHERE key = $1"
        } else {
            "INSERT INTO string_cache (key, value, date)
             VALUES ($1, $2, $3)"
        };
        self.execute(query, &[&cache_key, &result, &now]).unwrap();

        result
    }

    fn reset_all_contest_visibilities(&self) { self.execute("UPDATE contest SET public = $1", &[&false]).unwrap(); }
    fn reset_all_taskgroup_visibilities(&self) { self.execute("UPDATE taskgroup SET active = $1", &[&false]).unwrap(); }
}
