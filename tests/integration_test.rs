use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::str::contains;

#[test]
fn version_flag_returns_version_string() {
    let mut cmd = Command::cargo_bin("clew").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(contains("clew"))
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

const FIXTURE: &str = "---\nid: 42\nstatus: in_progress\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-25T14:30:00Z\n---\n\n# Add OAuth routes\n\n- [x] Scaffold handlers\n- [ ] Write tests\n";

fn make_project() -> assert_fs::TempDir {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child(".clew/increments").create_dir_all().unwrap();
    temp.child(".clew/archive").create_dir_all().unwrap();
    temp.child(".clew/increments/0042-add-oauth-routes.md")
        .write_str(FIXTURE)
        .unwrap();
    temp
}

#[test]
fn show_by_padded_id_outputs_file_verbatim() {
    let temp = make_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["show", "0042"])
        .assert()
        .success()
        .stdout(FIXTURE);
}

#[test]
fn show_by_unpadded_id() {
    let temp = make_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["show", "42"])
        .assert()
        .success()
        .stdout(FIXTURE);
}

#[test]
fn show_by_slug() {
    let temp = make_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["show", "add-oauth-routes"])
        .assert()
        .success()
        .stdout(FIXTURE);
}

#[test]
fn view_is_an_alias_for_show() {
    let temp = make_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["view", "42"])
        .assert()
        .success()
        .stdout(FIXTURE);
}

#[test]
fn show_finds_archived_increment() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child(".clew/increments").create_dir_all().unwrap();
    temp.child(".clew/archive").create_dir_all().unwrap();
    temp.child(".clew/archive/0007-old-work.md")
        .write_str(FIXTURE)
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["show", "7"])
        .assert()
        .success()
        .stdout(FIXTURE);
}

#[test]
fn show_walks_up_to_find_clew_root() {
    let temp = make_project();
    let nested = temp.child("a/b/c");
    nested.create_dir_all().unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(nested.path())
        .args(["show", "42"])
        .assert()
        .success()
        .stdout(FIXTURE);
}

#[test]
fn show_unknown_id_errors_with_exit_1() {
    let temp = make_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["show", "9999"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("not found"));
}

#[test]
fn show_outside_clew_project_errors() {
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["show", "1"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains(".clew/"));
}

// ---------------------------------------------------------------------------
// `clew new`
// ---------------------------------------------------------------------------

fn empty_project() -> assert_fs::TempDir {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child(".clew/increments").create_dir_all().unwrap();
    temp.child(".clew/archive").create_dir_all().unwrap();
    temp
}

fn read_increment(temp: &assert_fs::TempDir, filename: &str) -> String {
    std::fs::read_to_string(temp.path().join(".clew/increments").join(filename)).unwrap()
}

#[test]
fn new_creates_backlog_increment_with_padded_id() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Add OAuth"])
        .assert()
        .success()
        .stdout("0001\n");

    let body = read_increment(&temp, "0001-add-oauth.md");
    assert!(body.contains("id: 1"));
    assert!(body.contains("status: backlog"));
    assert!(body.contains("created_at: "));
    assert!(body.contains("updated_at: "));
}

#[test]
fn new_with_ready_flag_starts_in_todo() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Add OAuth", "--ready"])
        .assert()
        .success();

    let body = read_increment(&temp, "0001-add-oauth.md");
    assert!(body.contains("status: todo"));
}

#[test]
fn new_allocates_sequential_ids() {
    let temp = empty_project();
    for (title, expected_id) in [("First", "0001"), ("Second", "0002"), ("Third", "0003")] {
        Command::cargo_bin("clew")
            .unwrap()
            .current_dir(temp.path())
            .args(["new", title])
            .assert()
            .success()
            .stdout(format!("{expected_id}\n"));
    }
}

#[test]
fn new_id_allocation_includes_archive() {
    // Plan §"Allocation": archived IDs can be higher than active ones; the
    // counter must scan both subdirs so we never reuse an ID.
    let temp = empty_project();
    temp.child(".clew/archive/0007-old-work.md")
        .write_str("---\nid: 7\nstatus: done\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\n---\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Next"])
        .assert()
        .success()
        .stdout("0008\n");
}

#[test]
fn new_with_parent_writes_parent_field() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Parent epic"])
        .assert()
        .success();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Child increment", "--parent", "1"])
        .assert()
        .success();

    let body = read_increment(&temp, "0002-child-increment.md");
    assert!(body.contains("parent: 1"));
}

#[test]
fn new_with_missing_parent_errors() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Orphan", "--parent", "999"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("not found"))
        .stderr(contains("parent #0999"));
}

#[test]
fn new_slug_collision_in_increments_errors() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Add OAuth"])
        .assert()
        .success();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Add OAuth"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("slug 'add-oauth'"))
        .stderr(contains("0001-add-oauth.md"))
        .stderr(contains("try a more specific title"));
}

#[test]
fn new_slug_collision_against_archive_errors() {
    // Plan §"Slug rules": once a slug is used, it's reserved forever — the
    // collision check must span archive/ as well, otherwise reopen could
    // produce duplicates.
    let temp = empty_project();
    temp.child(".clew/archive/0001-add-oauth.md")
        .write_str("---\nid: 1\nstatus: done\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\n---\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Add OAuth"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("slug 'add-oauth'"));
}

#[test]
fn new_walks_up_to_find_clew_root() {
    let temp = empty_project();
    let nested = temp.child("a/b/c");
    nested.create_dir_all().unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(nested.path())
        .args(["new", "From subdir"])
        .assert()
        .success()
        .stdout("0001\n");

    assert!(temp
        .path()
        .join(".clew/increments/0001-from-subdir.md")
        .exists());
}

#[test]
fn new_outside_clew_project_errors() {
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Anything"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains(".clew/"));
}

// ---------------------------------------------------------------------------
// `clew list`
// ---------------------------------------------------------------------------

fn fixture_with(id: u32, slug: &str, status: &str, tags: Option<&str>) -> String {
    let tags_line = tags.map(|t| format!("tags: {t}\n")).unwrap_or_default();
    format!(
        "---\nid: {id}\nstatus: {status}\n{tags_line}created_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\n---\n# {slug}\n"
    )
}

fn write_increment(temp: &assert_fs::TempDir, subdir: &str, filename: &str, body: &str) {
    temp.child(format!(".clew/{subdir}/{filename}"))
        .write_str(body)
        .unwrap();
}

#[test]
fn list_default_shows_in_flight_only_sorted_by_id() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0002-second.md",
        &fixture_with(2, "second", "todo", None),
    );
    write_increment(
        &temp,
        "increments",
        "0001-first.md",
        &fixture_with(1, "first", "backlog", None),
    );
    write_increment(
        &temp,
        "archive",
        "0003-archived.md",
        &fixture_with(3, "archived", "done", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout("0001 backlog first\n0002 todo second\n");
}

#[test]
fn list_all_includes_archived() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-active.md",
        &fixture_with(1, "active", "todo", None),
    );
    write_increment(
        &temp,
        "archive",
        "0002-shipped.md",
        &fixture_with(2, "shipped", "done", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("0001 todo active\n0002 done shipped\n");
}

#[test]
fn list_filters_by_status() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );
    write_increment(
        &temp,
        "increments",
        "0002-b.md",
        &fixture_with(2, "b", "in_progress", None),
    );
    write_increment(
        &temp,
        "increments",
        "0003-c.md",
        &fixture_with(3, "c", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list", "--status", "todo"])
        .assert()
        .success()
        .stdout("0001 todo a\n0003 todo c\n");
}

#[test]
fn list_filters_by_tag() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", Some("[auth, p0]")),
    );
    write_increment(
        &temp,
        "increments",
        "0002-b.md",
        &fixture_with(2, "b", "todo", Some("[ui]")),
    );
    write_increment(
        &temp,
        "increments",
        "0003-c.md",
        &fixture_with(3, "c", "todo", Some("[auth]")),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list", "--tag", "auth"])
        .assert()
        .success()
        .stdout("0001 todo a\n0003 todo c\n");
}

#[test]
fn list_combines_tag_and_status_filters() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", Some("[auth]")),
    );
    write_increment(
        &temp,
        "increments",
        "0002-b.md",
        &fixture_with(2, "b", "in_progress", Some("[auth]")),
    );
    write_increment(
        &temp,
        "increments",
        "0003-c.md",
        &fixture_with(3, "c", "todo", Some("[ui]")),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list", "--tag", "auth", "--status", "todo"])
        .assert()
        .success()
        .stdout("0001 todo a\n");
}

#[test]
fn list_empty_project_succeeds_with_no_output() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout("");
}

#[test]
fn list_invalid_status_filter_errors() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list", "--status", "flying"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("invalid --status"))
        .stderr(contains("flying"));
}

#[test]
fn list_walks_up_to_find_clew_root() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-x.md",
        &fixture_with(1, "x", "todo", None),
    );
    let nested = temp.child("a/b");
    nested.create_dir_all().unwrap();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(nested.path())
        .args(["list"])
        .assert()
        .success()
        .stdout("0001 todo x\n");
}

#[test]
fn list_outside_clew_project_errors() {
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains(".clew/"));
}

#[test]
fn new_output_is_pipeable_to_show() {
    // The "stdout = data" principle: `clew new` prints just the ID so it can
    // feed into other commands that accept ID lookups.
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Pipeable"])
        .assert()
        .success();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["show", "0001"])
        .assert()
        .success()
        .stdout(contains("id: 1"));
}
