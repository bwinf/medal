use super::*;

use db_objects::{Contest, Task, Taskgroup};

use reqwest::StatusCode;
use std::path::Path;

fn addsimpleuser(conn: &mut rusqlite::Connection, username: String, password: String, is_t: bool, is_a: bool) {
    let mut test_user = conn.new_session("");
    test_user.username = Some(username);
    test_user.is_teacher = is_t;
    test_user.is_admin = Some(is_a);
    test_user.set_password(&password).expect("Set Password did not work correctly.");
    conn.save_session(test_user);
}

fn run<P, F>(p: P, f: F)
    where F: Fn(u16),
          P: Fn(&mut rusqlite::Connection) + std::marker::Send + 'static
{
    use std::sync::mpsc::channel;
    let (start_tx, start_rx) = channel();
    let (stop_tx, stop_rx) = channel();

    std::thread::spawn(move || {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        db_apply_migrations::test(&mut conn);

        p(&mut conn);

        // ID: 1, gets renamed
        let mut contest = Contest { id: None,
                                    location: "directory".to_string(),
                                    filename: "public.yaml".to_string(),
                                    name: "RenamedContestName".to_string(),
                                    duration: 1, // Time: 1 Minute
                                    public: true,
                                    start: None,
                                    end: None,
                                    review_start: None,
                                    review_end: None,
                                    min_grade: None,
                                    max_grade: None,
                                    positionalnumber: None,
                                    protected: false,
                                    requires_login: None,
                                    requires_contest: None,
                                    secret: None,
                                    message: None,
                                    image: None,
                                    language: None,
                                    category: None,
                                    standalone_task: None,
                                    tags: Vec::new(),
                                    taskgroups: Vec::new() };
        contest.save(&conn);

        // ID: 1
        let mut contest = Contest { id: None,
                                    location: "directory".to_string(),
                                    filename: "public.yaml".to_string(),
                                    name: "PublicContestName".to_string(),
                                    duration: 1, // Time: 1 Minute
                                    public: true,
                                    start: None,
                                    end: None,
                                    review_start: None,
                                    review_end: None,
                                    min_grade: None,
                                    max_grade: None,
                                    positionalnumber: None,
                                    protected: false,
                                    requires_login: None,
                                    requires_contest: None,
                                    secret: None,
                                    message: None,
                                    image: None,
                                    language: None,
                                    category: None,
                                    standalone_task: None,
                                    tags: Vec::new(),
                                    taskgroups: Vec::new() };
        let mut taskgroup = Taskgroup::new("TaskgroupName".to_string(), None);
        let task = Task::new("taskdir1".to_string(), None, 3); // ID: 1
        taskgroup.tasks.push(task);
        let task = Task::new("taskdir2".to_string(), None, 4); // ID: 2
        taskgroup.tasks.push(task);
        contest.taskgroups.push(taskgroup);
        contest.save(&conn);

        // ID: 2
        let mut contest = Contest { id: None,
                                    location: "directory".to_string(),
                                    filename: "private.yaml".to_string(),
                                    name: "PrivateContestName".to_string(),
                                    duration: 1, // Time: 1 Minute
                                    public: false,
                                    start: None,
                                    end: None,
                                    review_start: None,
                                    review_end: None,
                                    min_grade: None,
                                    max_grade: None,
                                    positionalnumber: None,
                                    protected: false,
                                    requires_login: None,
                                    requires_contest: None,
                                    secret: None,
                                    message: None,
                                    image: None,
                                    language: None,
                                    category: None,
                                    standalone_task: None,
                                    tags: Vec::new(),
                                    taskgroups: Vec::new() };
        let mut taskgroup = Taskgroup::new("TaskgroupName".to_string(), None);
        let task = Task::new("taskdir1".to_string(), None, 3); // ID: 3
        taskgroup.tasks.push(task);
        let task = Task::new("taskdir2".to_string(), None, 4); // ID: 4
        taskgroup.tasks.push(task);
        contest.taskgroups.push(taskgroup);
        contest.save(&conn);

        // ID: 3
        let mut contest = Contest { id: None,
                                    location: "directory".to_string(),
                                    filename: "infinite.yaml".to_string(),
                                    name: "InfiniteContestName".to_string(),
                                    duration: 0, // Time: Unlimited
                                    public: true,
                                    start: None,
                                    end: None,
                                    review_start: None,
                                    review_end: None,
                                    min_grade: None,
                                    max_grade: None,
                                    positionalnumber: None,
                                    protected: false,
                                    requires_login: None,
                                    requires_contest: None,
                                    secret: None,
                                    message: None,
                                    image: None,
                                    language: None,
                                    category: None,
                                    standalone_task: None,
                                    tags: Vec::new(),
                                    taskgroups: Vec::new() };
        let mut taskgroup = Taskgroup::new("TaskgroupRenameName".to_string(), None);
        let task = Task::new("taskdir1".to_string(), None, 3); // ID: 5
        taskgroup.tasks.push(task);
        let task = Task::new("taskdir2".to_string(), None, 4); // ID: 6
        taskgroup.tasks.push(task);
        contest.taskgroups.push(taskgroup);
        contest.save(&conn);

        let mut taskgroup = Taskgroup::new("TaskgroupNewName".to_string(), None);
        let task = Task::new("taskdir1".to_string(), None, 3); // ID: 5
        taskgroup.tasks.push(task);
        let task = Task::new("taskdir2".to_string(), None, 4); // ID: 6
        taskgroup.tasks.push(task);
        contest.taskgroups.push(taskgroup);
        contest.save(&conn);

        // ID: 4
        let mut contest = Contest { id: None,
                                    location: "directory".to_string(),
                                    filename: "publicround2.yaml".to_string(),
                                    name: "PublicContestRoundTwoName".to_string(),
                                    duration: 1, // Time: 1 Minute
                                    public: true,
                                    start: None,
                                    end: None,
                                    review_start: None,
                                    review_end: None,
                                    min_grade: None,
                                    max_grade: None,
                                    positionalnumber: None,
                                    protected: false,
                                    requires_login: None,
                                    requires_contest: Some("public.yaml".to_string()),
                                    secret: None,
                                    message: None,
                                    image: None,
                                    language: None,
                                    category: None,
                                    standalone_task: None,
                                    tags: Vec::new(),
                                    taskgroups: Vec::new() };
        let mut taskgroup = Taskgroup::new("TaskgroupName".to_string(), None);
        let task = Task::new("taskdir1".to_string(), None, 3); // ID: 7
        taskgroup.tasks.push(task);
        let task = Task::new("taskdir2".to_string(), None, 4); // ID: 8
        taskgroup.tasks.push(task);
        contest.taskgroups.push(taskgroup);
        contest.save(&conn);

        // Have the contest review start one day in the past
        let mut in_the_past = time::get_time();
        in_the_past.sec -= 60 * 60 * 24;

        // ID: 5
        let mut contest = Contest { id: None,
                                    location: "directory".to_string(),
                                    filename: "current_review.yaml".to_string(),
                                    name: "CurrentReviewContestName".to_string(),
                                    duration: 1, // Time: 1 Minute
                                    public: true,
                                    start: None,
                                    end: None,
                                    review_start: Some(in_the_past),
                                    review_end: None,
                                    min_grade: None,
                                    max_grade: None,
                                    positionalnumber: None,
                                    protected: false,
                                    requires_login: None,
                                    requires_contest: None,
                                    secret: None,
                                    message: None,
                                    image: None,
                                    language: None,
                                    category: None,
                                    standalone_task: None,
                                    tags: Vec::new(),
                                    taskgroups: Vec::new() };
        let mut taskgroup = Taskgroup::new("TaskgroupName".to_string(), None);
        let task = Task::new("taskdir1".to_string(), None, 3); // ID: 9
        taskgroup.tasks.push(task);
        let task = Task::new("taskdir2".to_string(), None, 4); // ID: 10
        taskgroup.tasks.push(task);
        contest.taskgroups.push(taskgroup);
        contest.save(&conn);

        // Have the contest review start one day in the future
        let mut in_the_future = time::get_time();
        in_the_future.sec += 60 * 60 * 24;

        // ID: 6
        let mut contest = Contest { id: None,
                                    location: "directory".to_string(),
                                    filename: "future_review.yaml".to_string(),
                                    name: "FutureReviewContestName".to_string(),
                                    duration: 1, // Time: 1 Minute
                                    public: true,
                                    start: None,
                                    end: None,
                                    review_start: Some(in_the_future),
                                    review_end: None,
                                    min_grade: None,
                                    max_grade: None,
                                    positionalnumber: None,
                                    protected: false,
                                    requires_login: None,
                                    requires_contest: None,
                                    secret: None,
                                    message: None,
                                    image: None,
                                    language: None,
                                    category: None,
                                    standalone_task: None,
                                    tags: Vec::new(),
                                    taskgroups: Vec::new() };
        let mut taskgroup = Taskgroup::new("TaskgroupName".to_string(), None);
        let task = Task::new("taskdir1".to_string(), None, 3); // ID: 11
        taskgroup.tasks.push(task);
        let task = Task::new("taskdir2".to_string(), None, 4); // ID: 12
        taskgroup.tasks.push(task);
        contest.taskgroups.push(taskgroup);
        contest.save(&conn);

        // ID: 7
        let mut contest = Contest { id: None,
                                    location: "directory".to_string(),
                                    filename: "protected.yaml".to_string(),
                                    name: "ProtectedContestName".to_string(),
                                    duration: 1, // Time: 1 Minute
                                    public: true,
                                    start: None,
                                    end: None,
                                    review_start: None,
                                    review_end: None,
                                    min_grade: None,
                                    max_grade: None,
                                    positionalnumber: None,
                                    protected: true,
                                    requires_login: None,
                                    requires_contest: None,
                                    secret: None,
                                    message: None,
                                    image: None,
                                    language: None,
                                    category: None,
                                    standalone_task: None,
                                    tags: Vec::new(),
                                    taskgroups: Vec::new() };
        let mut taskgroup = Taskgroup::new("TaskgroupName".to_string(), None);
        let task = Task::new("taskdir1".to_string(), None, 3); // ID: 13
        taskgroup.tasks.push(task);
        let task = Task::new("taskdir2".to_string(), None, 4); // ID: 14
        taskgroup.tasks.push(task);
        contest.taskgroups.push(taskgroup);
        contest.save(&conn);

        let mut config = config::read_config_from_file(Path::new("thisfileshoudnotexist.json"));

        let port = {
            use std::net::{Ipv6Addr, SocketAddrV6, TcpListener};
            TcpListener::bind(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)).unwrap().local_addr().unwrap().port()
        };

        config.port = Some(port);
        config.cookie_signing_secret = Some("testtesttesttesttesttesttesttest".to_string());
        let message = format!("Could not start server on port {}", port);
        let mut srvr = start_server(conn, config).expect(&message);

        // Message server started
        start_tx.send(port).unwrap();

        // Wait for test to finish
        stop_rx.recv().unwrap();

        srvr.close().unwrap();
    });

    // Wait for server to start
    let port: u16 = start_rx.recv().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Run test code
    f(port);

    // Message test finished
    stop_tx.send(()).unwrap();
}

fn login(port: u16, client: &reqwest::Client, username: &str, password: &str) -> reqwest::Response {
    let params = [("username", username), ("password", password)];
    client.post(&format!("http://localhost:{}/login", port)).form(&params).send().unwrap()
}

fn login_code(port: u16, client: &reqwest::Client, code: &str) -> reqwest::Response {
    let params = [("code", code)];
    client.post(&format!("http://localhost:{}/clogin", port)).form(&params).send().unwrap()
}

trait SimpleClient {
    fn pget(&self, port: u16, request: &str) -> reqwest::RequestBuilder;
    fn ppost(&self, port: u16, request: &str) -> reqwest::RequestBuilder;
}

impl SimpleClient for reqwest::Client {
    fn pget(&self, port: u16, request: &str) -> reqwest::RequestBuilder {
        self.get(&format!("http://localhost:{}/{}", port, request))
    }
    fn ppost(&self, port: u16, request: &str) -> reqwest::RequestBuilder {
        self.post(&format!("http://localhost:{}/{}", port, request))
    }
}

#[test]
fn start_server_and_check_requests() {
    run(|_| {},
        |port| {
            let mut resp = reqwest::get(&format!("http://localhost:{}", port)).unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Jugendwettbewerb Informatik</h1>"));
            assert!(!content.contains("Error"));
            assert!(!content.contains("Gruppenverwaltung"));

            let mut resp = reqwest::get(&format!("http://localhost:{}/contest", port)).unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<h1>Wettbewerbe</h1>"));
            assert!(!content.contains("Error"));

            let mut resp = reqwest::get(&format!("http://localhost:{}/group", port)).unwrap();
            let content = resp.text().unwrap();
            assert!(content.contains("<h1>Login</h1>"));
        })
}

#[test]
fn check_login_wrong_credentials() {
    run(|_| {},
        |port| {
            let client = reqwest::Client::new();

            let mut resp = login(port, &client, "nonexistingusername", "wrongpassword");
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<h1>Login</h1>"));
            assert!(content.contains("Login fehlgeschlagen."));
            assert!(!content.contains("Error"));

            let mut resp = login_code(port, &client, "g23AgaV");
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<h1>Login</h1>"));
            assert!(content.contains("Kein gültiger Code."));
            assert!(!content.contains("Error"));

            let mut resp = login_code(port, &client, "u9XuAbH7p");
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<h1>Login</h1>"));
            assert!(content.contains("Kein gültiger Code."));
            assert!(!content.contains("Error"));
        })
}

#[test]
fn check_login() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), false, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let mut resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let content = resp.text().unwrap();
            assert!(!content.contains("Error"));

            let mut set_cookie = resp.headers().get_all("Set-Cookie").iter();
            assert!(set_cookie.next().is_some());
            assert!(set_cookie.next().is_none());

            let location = resp.headers().get(reqwest::header::LOCATION).unwrap().to_str().unwrap();
            assert_eq!(location, format!("http://localhost:{}/", port));

            let mut resp = client.get(location).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(!content.contains("Error"));
            assert!(!content.contains("Gruppenverwaltung"));
            assert!(content.contains("Eingeloggt als <em>testusr</em>"));
            assert!(content.contains("Jugendwettbewerb Informatik</h1>"));
        })
}

#[test]
fn check_logout() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), false, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let resp = client.pget(port, "logout").send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Benutzername"));
            assert!(content.contains("Passwort"));
            assert!(content.contains("Gruppencode / Teilnahmecode"));
            assert!(content.contains("Jugendwettbewerb Informatik</h1>"));
        })
}

#[test]
fn check_group_creation_and_group_code_login() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), true, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("[Lehrer]"));
            assert!(content.contains("Gruppenverwaltung"));

            let mut resp = client.pget(port, "group/").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Gruppe anlegen"));

            let params = [("name", "WrongGroupname"), ("tag", "WrongMarker"), ("csrf_token", "76CfTPJaoz")];
            let resp = client.ppost(port, "group/").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FORBIDDEN);

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("name", "Groupname"), ("tag", "Marker"), ("csrf_token", csrf)];
            let resp = client.ppost(port, "group/").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "group/").send().unwrap();
            let content = resp.text().unwrap();
            assert!(!content.contains("WrongGroupname"));

            let pos = content.find("<td><a href=\"/group/1\">Groupname</a></td>").expect("Group not found");
            let groupcode = &content[pos + 58..pos + 66];

            // New client to test group code login
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login_code(port, &client, groupcode);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut set_cookie = resp.headers().get_all("Set-Cookie").iter();
            assert!(set_cookie.next().is_some());
            assert!(set_cookie.next().is_none());

            let location = resp.headers().get(reqwest::header::LOCATION).unwrap().to_str().unwrap();
            assert_eq!(location, format!("http://localhost:{}/profile?status=firstlogin", port));

            let mut resp = client.get(location).send().unwrap();
            let content = resp.text().unwrap();

            let pos = content.find("<p>Login-Code: ").expect("Logincode not found");
            let logincode = &content[pos + 15..pos + 25];

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("firstname", "FirstName"),
                          ("lastname", "LastName"),
                          ("grade", "8"),
                          ("sex", "2"),
                          ("csrf_token", csrf)];
            let resp = client.ppost(port, "profile").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let location = resp.headers().get(reqwest::header::LOCATION).unwrap().to_str().unwrap();
            assert_eq!(location, format!("http://localhost:{}/profile?status=DataChanged", port));

            // New client to test login code login
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login_code(port, &client, logincode);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let location = resp.headers().get(reqwest::header::LOCATION).unwrap().to_str().unwrap();
            assert_eq!(location, format!("http://localhost:{}/", port));

            let mut resp = client.get(location).send().unwrap();
            let content = resp.text().unwrap();
            assert!(content.contains("Eingeloggt als <em></em>"));
            println!("{}", content);
            assert!(content.contains("FirstName LastName"));
        })
}

#[test]
fn check_contest_start() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), false, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "contest/").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            println!("{}", content);
            assert!(content.contains("PublicContestName"));
            assert!(content.contains("InfiniteContestName"));
            assert!(!content.contains("PrivateContestName"));
            assert!(!content.contains("WrongContestName"));
            assert!(!content.contains("RenamedContestName"));
            assert!(content.contains("<a href=\"/contest/1\">PublicContestName</a>"));

            let mut resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("PublicContestName"));
            assert!(!content.contains("InfiniteContestName"));
            assert!(!content.contains("PrivateContestName"));
            assert!(!content.contains("WrongContestName"));
            assert!(!content.contains("RenamedContestName"));

            let params = [("csrf_token", "76CfTPJaoz")];
            let resp = client.ppost(port, "contest/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FORBIDDEN);

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "contest/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/1\">☆☆☆</a></li>"));
            assert!(content.contains("<a href=\"/task/2\">☆☆☆☆</a></li>"));
        })
}

#[test]
fn check_task_load_save() {
    run(|_| {},
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = client.pget(port, "contest/3").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let mut resp = client.pget(port, "task/5").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            let pos = content.find("#taskid=5&csrftoken=").expect("CSRF-Token not found");
            let csrf = &content[pos + 20..pos + 30];

            let mut resp = client.pget(port, "load/5").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let params = [("data", "WrongData"), ("grade", "1"), ("csrf_token", "FNQU4QsEMY")];
            let resp = client.ppost(port, "save/5").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FORBIDDEN);

            // Check that the illegitimate request did not actually change anything
            let mut resp = client.pget(port, "load/5").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "contest/3").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/5\">☆☆☆</a></li>"));
            assert!(content.contains("<a href=\"/task/6\">☆☆☆☆</a></li>"));

            let params = [("data", "SomeData"), ("grade", "67"), ("csrf_token", csrf)];
            let mut resp = client.ppost(port, "save/5").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "load/5").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "SomeData");

            let mut resp = client.pget(port, "contest/3").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/5\">★★☆</a></li>"));
            assert!(content.contains("<a href=\"/task/6\">☆☆☆☆</a></li>"));
        })
}

#[test]
fn check_task_load_save_logged_in() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), false, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "contest/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "task/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            let pos = content.find("#taskid=1&csrftoken=").expect("CSRF-Token not found");
            let csrf = &content[pos + 20..pos + 30];

            let mut resp = client.pget(port, "load/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let params = [("data", "WrongData"), ("grade", "1"), ("csrf_token", "FNQU4QsEMY")];
            let resp = client.ppost(port, "save/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FORBIDDEN);

            // Check that the illigal request did not actually change anything
            let mut resp = client.pget(port, "load/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/1\">☆☆☆</a></li>"));
            assert!(content.contains("<a href=\"/task/2\">☆☆☆☆</a></li>"));

            let params = [("data", "SomeData"), ("grade", "67"), ("csrf_token", csrf)];
            let mut resp = client.ppost(port, "save/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "load/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "SomeData");

            let mut resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/1\">★★☆</a></li>"));
            assert!(content.contains("<a href=\"/task/2\">☆☆☆☆</a></li>"));
        })
}

#[test]
fn check_taskgroup_rename() {
    run(|_| {},
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let mut resp = client.pget(port, "contest/3").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("TaskgroupNewName"));
            assert!(!content.contains("TaskgroupRenameName"));

            let mut resp = client.pget(port, "task/5").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("TaskgroupNewName"));
            assert!(!content.contains("TaskgroupRenameName"));
        })
}

#[test]
fn check_admin_interface_link() {
    run(|conn| {
            addsimpleuser(conn, "testadm".to_string(), "testpw1".to_string(), false, true);
            addsimpleuser(conn, "testusr".to_string(), "testpw2".to_string(), false, false);
            addsimpleuser(conn, "testtch".to_string(), "testpw3".to_string(), true, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testadm", "testpw1");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Administration"));
            assert!(content.contains("<a href=\"/admin/\""));

            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(!content.contains("Administration"));
            assert!(!content.contains("<a href=\"/admin/\""));

            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let mut resp = login(port, &client, "testusr", "testpw2");
            assert_eq!(resp.status(), StatusCode::FOUND);

            println!("{}", resp.text().unwrap());

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(!content.contains("Administration"));
            assert!(!content.contains("<a href=\"/admin/\""));

            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let mut resp = login(port, &client, "testtch", "testpw3");
            assert_eq!(resp.status(), StatusCode::FOUND);

            println!("{}", resp.text().unwrap());

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(!content.contains("Administration"));
            assert!(!content.contains("<a href=\"/admin/\""));
        })
}

#[test]
fn check_admin_interface_access() {
    run(|conn| {
            addsimpleuser(conn, "testadm".to_string(), "testpw1".to_string(), false, true);
            addsimpleuser(conn, "testusr".to_string(), "testpw2".to_string(), false, false);
            addsimpleuser(conn, "testtch".to_string(), "testpw3".to_string(), true, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testadm", "testpw1");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "admin").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Administration"));
            assert!(content.contains("Admin-Suche"));
            assert!(content.contains("Wettbewerbs-Export"));

            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let mut resp = client.pget(port, "admin").send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let content = resp.text().unwrap();
            assert!(!content.contains("Administration"));
            assert!(!content.contains("Admin-Suche"));
            assert!(!content.contains("Wettbewerbs-Export"));

            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let mut resp = login(port, &client, "testusr", "testpw2");
            assert_eq!(resp.status(), StatusCode::FOUND);

            println!("{}", resp.text().unwrap());

            let mut resp = client.pget(port, "admin").send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let content = resp.text().unwrap();
            assert!(!content.contains("Administration"));
            assert!(!content.contains("Admin-Suche"));
            assert!(!content.contains("Wettbewerbs-Export"));

            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let mut resp = login(port, &client, "testtch", "testpw3");
            assert_eq!(resp.status(), StatusCode::FOUND);

            println!("{}", resp.text().unwrap());

            let mut resp = client.pget(port, "admin").send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let content = resp.text().unwrap();
            assert!(!content.contains("Administration"));
            assert!(!content.contains("Admin-Suche"));
            assert!(!content.contains("Wettbewerbs-Export"));
        })
}

#[test]
fn check_admin_admission_upload() {
    run(|conn| {
            addsimpleuser(conn, "testadm".to_string(), "testpw1".to_string(), false, true);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testadm", "testpw1");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let resp = client.pget(port, "admin").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let mut resp = client.pget(port, "admin/contest/1/csv").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            // No data
            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "admin/contest/1/csv").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

            // Some data
            let params = [("csrf_token", csrf), ("admission_data", r#"[["20","ja"],["21","nein"],["18",""]]"#)];
            let resp = client.ppost(port, "admin/contest/1/csv").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            // Some more data
            let params = [("csrf_token", csrf),
                          ("admission_data",
                           r#"[["20","ja"],["21","nein"],["18",""],["30","ja"],["31","nein"],["28",""],["40","ja"],["41","nein"],["38",""],["12","anders"],
                               ["120","ja"],["121","nein"],["118",""],["130","ja"],["131","nein"],["128",""],["140","ja"],["141","nein"],["138",""],["112","anders"],
                               ["220","ja"],["221","nein"],["218",""],["230","ja"],["231","nein"],["228",""],["240","ja"],["241","nein"],["238",""],["212","anders"],
                               ["320","ja"],["321","nein"],["318",""],["330","ja"],["331","nein"],["328",""],["340","ja"],["341","nein"],["338",""],["312","anders"],
                               ["420","ja"],["421","nein"],["418",""]]"#)];
            let resp = client.ppost(port, "admin/contest/1/csv").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            // Invalid data
            let params = [("csrf_token", csrf), ("admission_data", r#"["blub"]"#)];
            let resp = client.ppost(port, "admin/contest/1/csv").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        })
}

#[test]
fn check_cleanup() {
    run(|conn| {
            let ago170days = Some(time::get_time() - time::Duration::days(170));
            let ago190days = Some(time::get_time() - time::Duration::days(190));

            let mut test_user = conn.new_session("");
            test_user.username = Some("testusr".to_string());
            test_user.set_password(&"testpw").expect("Set Password did not work correctly.");
            conn.session_set_activity_dates(test_user.id, ago190days, ago190days, ago190days);
            conn.save_session(test_user);

            let mut test_user = conn.new_session("");
            test_user.firstname = Some("firstname".to_string());
            test_user.lastname = Some("teststdold".to_string());
            test_user.logincode = Some("logincode1".to_string());
            test_user.managed_by = Some(1); // Fake id, should this group really exist?
            conn.session_set_activity_dates(test_user.id, ago190days, ago190days, ago190days);
            conn.save_session(test_user);

            let mut test_user = conn.new_session("");
            test_user.firstname = Some("firstname".to_string());
            test_user.lastname = Some("teststdnew".to_string());
            test_user.logincode = Some("logincode2".to_string());
            test_user.managed_by = Some(1);
            conn.session_set_activity_dates(test_user.id, ago190days, ago170days, ago190days);
            conn.save_session(test_user);

            addsimpleuser(conn, "testadm".to_string(), "testpw1".to_string(), false, true);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();
            // Login as Admin
            let resp = login(port, &client, "testadm", "testpw1");
            assert_eq!(resp.status(), StatusCode::FOUND);

            // Check old account still existing
            let mut resp = client.pget(port, "admin/user/2").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("teststdold"));

            // Delete old data
            let mut resp = client.pget(port, "admin/cleanup").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Alte Daten löschen"));

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("csrf_token", csrf)];
            let mut resp = client.ppost(port, "admin/cleanup/hard").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{\"status\":\"ok\",\"n_user\":1,\"n_group\":0,\"n_teacher\":0,\"n_other\":0}\n");

            // Check old account no longer existing
            let mut resp = client.pget(port, "admin/user/2").send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let content = resp.text().unwrap();
            assert!(!content.contains("teststdold"));

            // Logout as admin
            let resp = client.pget(port, "logout").send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            // Check login with old account no longer possible
            let resp = login_code(port, &client, "logincode1");
            assert_eq!(resp.status(), StatusCode::OK);

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
            let content = resp.text().unwrap();
            assert!(!content.contains("Eingeloggt als "));
            assert!(!content.contains("teststdold"));

            let resp = client.pget(port, "logout").send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            // Check login with new account still possible
            let resp = login_code(port, &client, "logincode2");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
            let content = resp.text().unwrap();
            assert!(content.contains("Eingeloggt als "));
            assert!(content.contains("teststdnew"));

            let resp = client.pget(port, "logout").send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            // Check login with new account still possible
            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
            let content = resp.text().unwrap();
            assert!(content.contains("Eingeloggt als <em>testusr</em>"));

            let resp = client.pget(port, "logout").send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);
        })
}

#[test]
fn check_contest_requirement() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), false, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            // Check contest can not be started
            let mut resp = client.pget(port, "contest/4").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
            let content = resp.text().unwrap();

            assert!(!content.contains("csrf_token"));
            assert!(!content.contains("<a href=\"/task/7\">☆☆☆</a></li>"));
            assert!(!content.contains("<a href=\"/task/8\">☆☆☆☆</a></li>"));

            // Start other contest
            let mut resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
            let content = resp.text().unwrap();

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "contest/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/1\">☆☆☆</a></li>"));
            assert!(content.contains("<a href=\"/task/2\">☆☆☆☆</a></li>"));

            // Check contest can be started now
            let mut resp = client.pget(port, "contest/4").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
            let content = resp.text().unwrap();

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "contest/4").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "contest/4").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/7\">☆☆☆</a></li>"));
            assert!(content.contains("<a href=\"/task/8\">☆☆☆☆</a></li>"));
        })
}

#[test]
fn check_group_creation_and_group_code_login_no_data() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), true, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("[Lehrer]"));
            assert!(content.contains("Gruppenverwaltung"));

            let mut resp = client.pget(port, "group/").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Gruppe anlegen"));

            let params = [("name", "WrongGroupname"), ("tag", "WrongMarker"), ("csrf_token", "76CfTPJaoz")];
            let resp = client.ppost(port, "group/").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FORBIDDEN);

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("name", "Groupname"), ("tag", "Marker"), ("csrf_token", csrf)];
            let resp = client.ppost(port, "group/").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "group/").send().unwrap();
            let content = resp.text().unwrap();
            assert!(!content.contains("WrongGroupname"));

            let pos = content.find("<td><a href=\"/group/1\">Groupname</a></td>").expect("Group not found");
            let groupcode = &content[pos + 58..pos + 66];

            // New client to test group code login
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login_code(port, &client, groupcode);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut set_cookie = resp.headers().get_all("Set-Cookie").iter();
            assert!(set_cookie.next().is_some());
            assert!(set_cookie.next().is_none());

            let location = resp.headers().get(reqwest::header::LOCATION).unwrap().to_str().unwrap();
            assert_eq!(location, format!("http://localhost:{}/profile?status=firstlogin", port));

            let mut resp = client.get(location).send().unwrap();
            let content = resp.text().unwrap();

            let pos = content.find("<p>Login-Code: ").expect("Logincode not found");
            let logincode = &content[pos + 15..pos + 25];

            // New client to test login code login
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login_code(port, &client, logincode);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let location = resp.headers().get(reqwest::header::LOCATION).unwrap().to_str().unwrap();
            assert_eq!(location, format!("http://localhost:{}/", port));

            // Client is forwarded to login page?
            let resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let location = resp.headers().get(reqwest::header::LOCATION).unwrap().to_str().unwrap();
            assert_eq!(location, format!("http://localhost:{}/profile?status=firstlogin", port));
        })
}

#[test]
fn check_contest_timelimit_student() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), false, false);

            let mut more_than_one_minute_ago = time::get_time();
            // Have the contest started more than a minute ago.
            more_than_one_minute_ago.sec -= 90;
            conn.execute("INSERT INTO participation (contest, session, start_date)
                          SELECT $1, id, $2 FROM session WHERE username = 'testusr'",
                         &[&1, &more_than_one_minute_ago])
                .unwrap();
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let resp = client.pget(port, "task/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "profile").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let mut resp = client.pget(port, "load/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let params = [("data", "SomeData"), ("grade", "67"), ("csrf_token", csrf)];
            let resp = client.ppost(port, "save/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let mut resp = client.pget(port, "load/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/1\">☆☆☆</a></li>"));
            assert!(content.contains("<a href=\"/task/2\">☆☆☆☆</a></li>"));
        })
}

#[test]
fn check_contest_timelimit_teacher() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), true, false);

            let mut now = time::get_time();
            now.sec -= 90; // Have the contest started more than a minute ago.
            conn.execute("INSERT INTO participation (contest, session, start_date)
                          SELECT $1, id, $2 FROM session WHERE username = 'testusr'",
                         &[&1, &now])
                .unwrap();
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let resp = client.pget(port, "task/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let mut resp = client.pget(port, "profile").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let mut resp = client.pget(port, "load/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let params = [("data", "SomeData"), ("grade", "67"), ("csrf_token", csrf)];
            let mut resp = client.ppost(port, "save/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "load/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "SomeData");

            let mut resp = client.pget(port, "contest/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/1\">★★☆</a></li>"));
            assert!(content.contains("<a href=\"/task/2\">☆☆☆☆</a></li>"));
        })
}

#[test]
fn check_review_student() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), false, false);

            let mut more_than_one_minute_ago = time::get_time();
            // Have the contest started more than a minute ago.
            more_than_one_minute_ago.sec -= 90;
            conn.execute("INSERT INTO participation (contest, session, start_date)
                          SELECT $1, id, $2 FROM session WHERE username = 'testusr'",
                         &[&5, &more_than_one_minute_ago])
                .unwrap();
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "contest/5").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Review-Modus: Du kannst die Aufgaben öffnen und bearbeiten."));

            let mut resp = client.pget(port, "task/9").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<em>Review-Modus</em>"));

            let mut resp = client.pget(port, "profile").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let mut resp = client.pget(port, "load/9").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let params = [("data", "SomeData"), ("grade", "67"), ("csrf_token", csrf)];
            let resp = client.ppost(port, "save/9").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let mut resp = client.pget(port, "load/9").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "contest/5").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/9\">☆☆☆</a></li>"));
            assert!(content.contains("<a href=\"/task/10\">☆☆☆☆</a></li>"));
        })
}

#[test]
fn check_future_review_student() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), false, false);

            let mut more_than_one_minute_ago = time::get_time();
            // Have the contest started more than a minute ago.
            more_than_one_minute_ago.sec -= 90;
            conn.execute("INSERT INTO participation (contest, session, start_date)
                          SELECT $1, id, $2 FROM session WHERE username = 'testusr'",
                         &[&6, &more_than_one_minute_ago])
                .unwrap();
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "contest/6").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Der Review-Modus beginnt in"));

            let resp = client.pget(port, "task/11").send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "profile").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let mut resp = client.pget(port, "load/11").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let params = [("data", "SomeData"), ("grade", "67"), ("csrf_token", csrf)];
            let resp = client.ppost(port, "save/11").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let mut resp = client.pget(port, "load/11").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "contest/6").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("<a href=\"/task/11\">☆☆☆</a></li>"));
            assert!(content.contains("<a href=\"/task/12\">☆☆☆☆</a></li>"));
        })
}

#[test]
fn check_teacher_admin_review() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), true, false);
            addsimpleuser(conn, "testusr2".to_string(), "testpw2".to_string(), true, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let mut resp = client.pget(port, "group/").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("name", "Groupname"), ("tag", "Marker"), ("csrf_token", csrf)];
            let resp = client.ppost(port, "group/").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "group/").send().unwrap();
            let content = resp.text().unwrap();
            let pos = content.find("<td><a href=\"/group/1\">Groupname</a></td>").expect("Group not found");
            let groupcode = &content[pos + 58..pos + 66];

            let resp = login_code(port, &client, groupcode);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "profile").send().unwrap();
            let content = resp.text().unwrap();
            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("firstname", "A"), ("lastname", "B"), ("grade", "1"), ("sex", ""), ("csrf_token", csrf)];
            let resp = client.ppost(port, "profile").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            // Start contest
            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "contest/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let params = [("data", "SomeData"), ("grade", "67"), ("csrf_token", csrf)];
            let mut resp = client.ppost(port, "save/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "load/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "SomeData");

            let mut resp = client.pget(port, "load/1?submission=1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "SomeData");

            let resp = login_code(port, &client, groupcode);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "profile").send().unwrap();
            let content = resp.text().unwrap();
            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];
            let params = [("firstname", "A"), ("lastname", "B"), ("grade", "1"), ("sex", ""), ("csrf_token", csrf)];
            let resp = client.ppost(port, "profile").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            // Start contest
            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "contest/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let params = [("data", "SomeData"), ("grade", "67"), ("csrf_token", csrf)];
            let mut resp = client.ppost(port, "save/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let mut resp = client.pget(port, "load/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "SomeData");

            let mut resp = client.pget(port, "load/1?submission=1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST); // TODO: Should this not be "UNAUTHORIZED"?

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "load/1?submission=1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert_eq!(content, "SomeData");

            let resp = login(port, &client, "testusr2", "testpw2");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "load/1?submission=1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST); // TODO: Should this not be "UNAUTHORIZED"?

            let content = resp.text().unwrap();
            assert_eq!(content, "{}");
        })
}

fn sim_create_group(client: &reqwest::Client, port: u16, name: &str) -> (String, String) {
    let mut resp = client.pget(port, "group/").send().unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let content = resp.text().unwrap();
    let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
    let csrf = &content[pos + 39..pos + 49];
    let params = [("name", name), ("tag", ""), ("csrf_token", csrf)];
    let resp = client.ppost(port, "group/").form(&params).send().unwrap();
    assert_eq!(resp.status(), StatusCode::FOUND);

    let mut resp = client.pget(port, "group/").send().unwrap();
    let content = resp.text().unwrap();
    let pos = content.find(&format!("\">{}</a></td>", name)).expect("Group name not found");
    let groupcode = &content[(pos + 28 + name.len())..(pos + 36 + name.len())];

    let ipos = content[..pos].rfind("<td><a href=\"/group/").expect("Group link not found");
    let group_id = &content[(ipos + 20)..pos];

    assert!(group_id.len() < 5); // Group ID should not be larger than 9999

    (group_id.to_string(), groupcode.to_string())
}

fn sim_login_groupcode(client: &reqwest::Client, port: u16, groupcode: &str, data: (&str, &str, &str, &str)) -> String {
    let resp = login_code(port, &client, &groupcode);
    assert_eq!(resp.status(), StatusCode::FOUND);

    let mut resp = client.pget(port, "profile").send().unwrap();
    let content = resp.text().unwrap();
    let pos = content.find("<p>Login-Code: ").expect("Login code not found");
    let logincode = &content[pos + 15..pos + 25];

    let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
    let csrf = &content[pos + 39..pos + 49];
    let params =
        [("firstname", data.0), ("lastname", data.1), ("grade", data.2), ("sex", data.3), ("csrf_token", csrf)];
    let resp = client.ppost(port, "profile").form(&params).send().unwrap();
    assert_eq!(resp.status(), StatusCode::FOUND);

    logincode.to_string()
}

fn sim_start_contest(client: &reqwest::Client, port: u16, contest_id: &str) {
    let mut resp = client.pget(port, &format!("contest/{}", contest_id)).send().unwrap();
    let content = resp.text().unwrap();

    let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
    let csrf = &content[pos + 39..pos + 49];

    let params = [("csrf_token", csrf)];
    let resp = client.ppost(port, &format!("contest/{}", contest_id)).form(&params).send().unwrap();
    assert_eq!(resp.status(), StatusCode::FOUND);
}

fn sim_participate(client: &reqwest::Client, port: u16, task_id: &str, data: (&str, &str)) {
    let mut resp = client.pget(port, "profile").send().unwrap();
    let content = resp.text().unwrap();

    let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
    let csrf = &content[pos + 39..pos + 49];

    let params = [("data", data.0), ("grade", data.1), ("csrf_token", csrf)];
    let mut resp = client.ppost(port, &format!("save/{}", task_id)).form(&params).send().unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let content = resp.text().unwrap();
    assert_eq!(content, "{}");

    let mut resp = client.pget(port, &format!("load/{}", task_id)).send().unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let content = resp.text().unwrap();
    assert_eq!(content, data.0);
}

#[test]
fn check_teacher_can_delete_users_and_groups() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), true, false);
            addsimpleuser(conn, "testusr2".to_string(), "testpw2".to_string(), true, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let (_group_id, groupcode) = sim_create_group(&client, port, "Groupname");

            let logincode = sim_login_groupcode(&client, port, &groupcode, ("Test", "Student", "1", ""));

            sim_start_contest(&client, port, "1");
            sim_participate(&client, port, "1", ("SomeData", "67"));

            let logincode2 = sim_login_groupcode(&client, port, &groupcode, ("Test2", "Student2", "2", ""));

            sim_start_contest(&client, port, "1");
            sim_participate(&client, port, "1", ("SomeData", "68"));

            let resp = login(port, &client, "testusr2", "testpw2");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "profile").send().unwrap();
            let content = resp.text().unwrap();

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let resp = client.pget(port, "admin/user/3").send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "admin/user/3").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let resp = client.pget(port, "admin/group/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "admin/group/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

            let resp = login_code(port, &client, &logincode);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let resp = login_code(port, &client, &logincode2);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "profile").send().unwrap();
            let content = resp.text().unwrap();

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let resp = client.pget(port, "admin/user/3").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "admin/user/3").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let resp = client.pget(port, "admin/group/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "admin/group/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let resp = login_code(port, &client, &logincode);
            assert_eq!(resp.status(), StatusCode::OK); // TODO: Maybe we should return UNAUTHORIZED on failing login

            let resp = login_code(port, &client, &logincode2);
            assert_eq!(resp.status(), StatusCode::OK);
        })
}

#[test]
fn check_teacher_can_not_delete_protected_users_and_groups() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), true, false);
            addsimpleuser(conn, "testusr2".to_string(), "testpw2".to_string(), true, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let (_group_id, groupcode) = sim_create_group(&client, port, "Groupname");

            let logincode = sim_login_groupcode(&client, port, &groupcode, ("Test", "Student", "1", ""));

            sim_start_contest(&client, port, "7");
            sim_participate(&client, port, "13", ("SomeData", "67"));

            let logincode2 = sim_login_groupcode(&client, port, &groupcode, ("Test2", "Student2", "2", ""));

            sim_start_contest(&client, port, "7");
            sim_participate(&client, port, "13", ("SomeData", "68"));

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "profile").send().unwrap();
            let content = resp.text().unwrap();

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let resp = client.pget(port, "admin/user/3").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "admin/user/3").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK); // TODO: Maybe we should return something else on failing delete

            let resp = client.pget(port, "admin/group/1").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let params = [("csrf_token", csrf)];
            let resp = client.ppost(port, "admin/group/1").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK); // TODO: Maybe we should return something else on failing delete

            let resp = login_code(port, &client, &logincode);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let resp = login_code(port, &client, &logincode2);
            assert_eq!(resp.status(), StatusCode::FOUND);
        })
}

#[test]
fn check_group_csv_upload_and_login() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), true, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("[Lehrer]"));
            assert!(content.contains("Gruppenverwaltung"));

            let mut resp = client.pget(port, "group/csv").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Gruppen per CSV-Upload anlegen"));

            let wrong_group_data  = "[[\"WrongGroupname\",\"7\",\"Gabi\",\"Musterfrau\",\"m\"],[\"WrongGroupname\",\"7\",\"Max\",\"Mustermann\",\"f\"],[\"WrongGroupname2\",\"12\",\"Ferdinand\",\"Fallbeispiel\",\"d\"]]";

            let params = [("group_data", wrong_group_data), ("gymnasium", "g8"), ("csrf_token", "76CfTPJaoz")];
            let resp = client.ppost(port, "group/csv").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FORBIDDEN);

            let group_data  = "[[\"7a\",\"7\",\"Gabi\",\"Musterfrau\",\"m\"],[\"7a\",\"7\",\"Max\",\"Mustermann\",\"f\"],[\"Info19\",\"12\",\"Ferdinand\",\"Fallbeispiel\",\"d\"]]";

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let params = [("group_data", group_data), ("gymnasium", "g8"), ("csrf_token", csrf)];
            let resp = client.ppost(port, "group/csv").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "group/").send().unwrap();
            let content = resp.text().unwrap();
            assert!(!content.contains("WrongGroupname"));

            content.find("<td><a href=\"/group/1\">7a</a></td>").expect("Group not found");

            let mut resp = client.pget(port, "group/1").send().unwrap();
            let content = resp.text().unwrap();

            println!("{}", content);

            let pos = content.find("<td><a href=\"/admin/user/2\">Gabi Musterfrau</a></td>").expect("User not found");
            let logincode = &content[pos + 66..pos + 76];
            let grade = &content[pos + 95..pos + 96];

            assert_eq!(grade, "7");

            // New client to test login code login
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login_code(port, &client, logincode);
            assert_eq!(resp.status(), StatusCode::FOUND);

            let location = resp.headers().get(reqwest::header::LOCATION).unwrap().to_str().unwrap();
            assert_eq!(location, format!("http://localhost:{}/", port));

            let mut resp = client.get(location).send().unwrap();
            let content = resp.text().unwrap();
            assert!(content.contains("Eingeloggt als <em></em>"));
            assert!(content.contains("Gabi Musterfrau"));
        })
}

#[test]
fn check_group_csv_upload_update() {
    run(|conn| {
            addsimpleuser(conn, "testusr".to_string(), "testpw".to_string(), true, false);
        },
        |port| {
            let client = reqwest::Client::builder().cookie_store(true)
                                                   .redirect(reqwest::RedirectPolicy::none())
                                                   .build()
                                                   .unwrap();

            let resp = login(port, &client, "testusr", "testpw");
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("[Lehrer]"));
            assert!(content.contains("Gruppenverwaltung"));

            let mut resp = client.pget(port, "group/csv").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Gruppen per CSV-Upload anlegen"));

            let group_data  = "[[\"7a\",\"7\",\"Gabi\",\"Musterfrau\",\"m\"],[\"7a\",\"7\",\"Max\",\"Mustermann\",\"f\"],[\"Info19\",\"12\",\"Ferdinand\",\"Fallbeispiel\",\"d\"]]";

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let params = [("group_data", group_data), ("gymnasium", "g8"), ("csrf_token", csrf)];
            let resp = client.ppost(port, "group/csv").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "group/").send().unwrap();
            let content = resp.text().unwrap();
            assert!(!content.contains("WrongGroupname"));

            content.find("<td><a href=\"/group/1\">7a</a></td>").expect("Group not found");

            let mut resp = client.pget(port, "group/1").send().unwrap();
            let content = resp.text().unwrap();

            let pos = content.find("<td><a href=\"/admin/user/2\">Gabi Musterfrau</a></td>").expect("User not found");
            let logincode = &content[pos + 66..pos + 76];
            let grade = &content[pos + 95..pos + 96];

            assert_eq!(grade, "7");

            let mut resp = client.pget(port, "group/csv").send().unwrap();
            assert_eq!(resp.status(), StatusCode::OK);

            let content = resp.text().unwrap();
            assert!(content.contains("Gruppen per CSV-Upload anlegen"));

            let group_data  = "[[\"7a\",\"3\",\"Gabi\",\"Musterfrau\",\"m\"],[\"7a\",\"3\",\"Max\",\"Mustermann\",\"f\"],[\"Info19\",\"11\",\"Ferdinand\",\"Fallbeispiel\",\"d\"]]";

            let pos = content.find("type=\"hidden\" name=\"csrf_token\" value=\"").expect("CSRF-Token not found");
            let csrf = &content[pos + 39..pos + 49];

            let params = [("group_data", group_data), ("gymnasium", "g8"), ("csrf_token", csrf)];
            let resp = client.ppost(port, "group/csv").form(&params).send().unwrap();
            assert_eq!(resp.status(), StatusCode::FOUND);

            let mut resp = client.pget(port, "group/").send().unwrap();
            let content = resp.text().unwrap();
            assert!(!content.contains("WrongGroupname"));

            content.find("<td><a href=\"/group/1\">7a</a></td>").expect("Group not found");

            let mut resp = client.pget(port, "group/1").send().unwrap();
            let content = resp.text().unwrap();

            let pos = content.find("<td><a href=\"/admin/user/2\">Gabi Musterfrau</a></td>").expect("User not found");
            let logincode2 = &content[pos + 66..pos + 76];
            let grade2 = &content[pos + 95..pos + 96];

            assert_eq!(grade2, "3");
            assert_eq!(logincode2, logincode);
        })
}
