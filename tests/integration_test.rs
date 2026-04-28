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

#[test]
fn top_level_help_lists_new_command_with_usage() {
    let mut cmd = Command::cargo_bin("clew").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains("Usage: clew [COMMAND]"))
        .stdout(contains("new       Create a new increment"));
}

#[test]
fn new_help_documents_arguments_and_flags() {
    let mut cmd = Command::cargo_bin("clew").unwrap();
    cmd.args(["new", "--help"])
        .assert()
        .success()
        .stdout(contains("Usage: clew new [OPTIONS] <TITLE>"))
        .stdout(contains("<TITLE>"))
        .stdout(contains("Increment title"))
        .stdout(contains("--ready"))
        .stdout(contains("Create the increment as todo instead of backlog"))
        .stdout(contains("--parent <PARENT>"))
        .stdout(contains("Parent increment ID"));
}

// ---------------------------------------------------------------------------
// `clew init`
// ---------------------------------------------------------------------------

const INIT_CREATED_STDERR: &str = "created: .clew\ncreated: .clew/increments\ncreated: .clew/archive\ncreated: .clew/path.md\ncreated: .clew/README.md\n";
const INIT_EXISTS_STDERR: &str = "exists: .clew\nexists: .clew/increments\nexists: .clew/archive\nexists: .clew/path.md\nexists: .clew/README.md\n";

#[test]
fn init_creates_expected_layout() {
    let temp = assert_fs::TempDir::new().unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("init")
        .assert()
        .success()
        .stdout("")
        .stderr(INIT_CREATED_STDERR);

    temp.child(".clew").assert(predicates::path::is_dir());
    temp.child(".clew/increments")
        .assert(predicates::path::is_dir());
    temp.child(".clew/archive")
        .assert(predicates::path::is_dir());
    temp.child(".clew/path.md")
        .assert(predicates::path::is_file());
    temp.child(".clew/README.md")
        .assert(predicates::path::is_file());
    assert_eq!(
        std::fs::read_to_string(temp.path().join(".clew/path.md")).unwrap(),
        ""
    );
}

#[test]
fn init_rerun_reports_existing_and_does_not_overwrite() {
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("init")
        .assert()
        .success();
    temp.child(".clew/path.md")
        .write_str("keep path\n")
        .unwrap();
    temp.child(".clew/README.md")
        .write_str("keep readme\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("init")
        .assert()
        .success()
        .stdout("")
        .stderr(INIT_EXISTS_STDERR);

    assert_eq!(
        std::fs::read_to_string(temp.path().join(".clew/path.md")).unwrap(),
        "keep path\n"
    );
    assert_eq!(
        std::fs::read_to_string(temp.path().join(".clew/README.md")).unwrap(),
        "keep readme\n"
    );
}

#[test]
fn init_repairs_partial_state_without_touching_existing_files() {
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("init")
        .assert()
        .success();
    std::fs::remove_dir_all(temp.path().join(".clew/archive")).unwrap();
    temp.child(".clew/path.md")
        .write_str("keep path\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("init")
        .assert()
        .success()
        .stdout("")
        .stderr("exists: .clew\nexists: .clew/increments\ncreated: .clew/archive\nexists: .clew/path.md\nexists: .clew/README.md\n");

    temp.child(".clew/archive")
        .assert(predicates::path::is_dir());
    assert_eq!(
        std::fs::read_to_string(temp.path().join(".clew/path.md")).unwrap(),
        "keep path\n"
    );
}

#[test]
fn init_readme_matches_snapshot() {
    let temp = assert_fs::TempDir::new().unwrap();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("init")
        .assert()
        .success();

    let readme = std::fs::read_to_string(temp.path().join(".clew/README.md")).unwrap();
    insta::assert_snapshot!(readme);
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

fn increment_body(contents: &str) -> &str {
    let close = contents.find("\n---\n").unwrap();
    &contents[close + "\n---\n".len()..]
}

#[test]
fn new_creates_backlog_increment_with_padded_id() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Add OAuth"])
        .write_stdin("")
        .assert()
        .success()
        .stdout("0001\n");

    let contents = read_increment(&temp, "0001-add-oauth.md");
    assert!(contents.contains("id: 1"));
    assert!(contents.contains("status: backlog"));
    assert!(contents.contains("created_at: "));
    assert!(contents.contains("updated_at: "));
    assert_eq!(increment_body(&contents), "# Add OAuth\n\n");
}

#[test]
fn new_reads_piped_stdin_body_verbatim() {
    let temp = empty_project();
    let stdin = "## Context\nbody line\n";

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "With body"])
        .write_stdin(stdin)
        .assert()
        .success()
        .stdout("0001\n");

    let contents = read_increment(&temp, "0001-with-body.md");
    assert_eq!(increment_body(&contents), stdin);
}

#[test]
fn new_reads_heredoc_equivalent_stdin_body() {
    let temp = empty_project();
    let stdin = "## Context\n\n- [ ] First task\n- [ ] Second task\n";

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Heredoc body"])
        .write_stdin(stdin)
        .assert()
        .success();

    let contents = read_increment(&temp, "0001-heredoc-body.md");
    assert_eq!(increment_body(&contents), stdin);
}

#[test]
fn new_with_empty_stdin_writes_title_heading() {
    let temp = empty_project();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Empty stdin"])
        .write_stdin("")
        .assert()
        .success();

    let contents = read_increment(&temp, "0001-empty-stdin.md");
    assert_eq!(increment_body(&contents), "# Empty stdin\n\n");
}

#[test]
fn new_preserves_leading_whitespace_in_stdin_body() {
    let temp = empty_project();
    let stdin = "  indented first line\n\tTabbed second line\n";

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Whitespace body"])
        .write_stdin(stdin)
        .assert()
        .success();

    let contents = read_increment(&temp, "0001-whitespace-body.md");
    assert_eq!(increment_body(&contents), stdin);
}

#[test]
fn new_rejects_stdin_starting_with_lf_frontmatter_delimiter() {
    let temp = empty_project();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Bad body"])
        .write_stdin("---\nid: 1\n---\n")
        .assert()
        .failure()
        .code(1)
        .stderr(contains("stdin appears to contain frontmatter"))
        .stderr(contains("pass body content only"));

    assert!(!temp
        .path()
        .join(".clew/increments/0001-bad-body.md")
        .exists());
}

#[test]
fn new_rejects_stdin_starting_with_crlf_frontmatter_delimiter() {
    let temp = empty_project();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Bad body"])
        .write_stdin("---\r\nid: 1\r\n---\r\n")
        .assert()
        .failure()
        .code(1)
        .stderr(contains("stdin appears to contain frontmatter"));

    assert!(!temp
        .path()
        .join(".clew/increments/0001-bad-body.md")
        .exists());
}

#[test]
fn new_allows_frontmatter_delimiter_later_in_stdin_body() {
    let temp = empty_project();
    let stdin = "Intro\n\n---\n\nThematic break is allowed.\n";

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Allowed delimiter"])
        .write_stdin(stdin)
        .assert()
        .success();

    let contents = read_increment(&temp, "0001-allowed-delimiter.md");
    assert_eq!(increment_body(&contents), stdin);
}

#[test]
fn new_without_stdin_in_test_harness_writes_title_heading() {
    // assert_cmd does not allocate a TTY, so this covers the common agent/test
    // harness path where stdin is non-TTY but empty. The interactive TTY path
    // should be manually verified with `./clew new "Manual title heading"`.
    let temp = empty_project();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "No redirected stdin"])
        .write_stdin("")
        .assert()
        .success();

    let contents = read_increment(&temp, "0001-no-redirected-stdin.md");
    assert_eq!(increment_body(&contents), "# No redirected stdin\n\n");
}

#[test]
fn new_with_ready_flag_starts_in_todo() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Add OAuth", "--ready"])
        .write_stdin("")
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
            .write_stdin("")
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
        .write_stdin("")
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
        .write_stdin("")
        .assert()
        .success();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Child increment", "--parent", "1"])
        .write_stdin("")
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
        .write_stdin("")
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
        .write_stdin("")
        .assert()
        .success();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["new", "Add OAuth"])
        .write_stdin("")
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
        .write_stdin("")
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
        .write_stdin("")
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
        .write_stdin("")
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
fn list_default_excludes_terminal_statuses_even_if_unarchived() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-active.md",
        &fixture_with(1, "active", "todo", None),
    );
    write_increment(
        &temp,
        "increments",
        "0002-done-but-not-archived.md",
        &fixture_with(2, "done-but-not-archived", "done", None),
    );
    write_increment(
        &temp,
        "increments",
        "0003-abandoned-but-not-archived.md",
        &fixture_with(3, "abandoned-but-not-archived", "abandoned", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout("0001 todo active\n");
}

#[test]
fn list_all_includes_archived_and_terminal_statuses() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-active.md",
        &fixture_with(1, "active", "todo", None),
    );
    write_increment(
        &temp,
        "increments",
        "0002-done-but-not-archived.md",
        &fixture_with(2, "done-but-not-archived", "done", None),
    );
    write_increment(
        &temp,
        "archive",
        "0003-shipped.md",
        &fixture_with(3, "shipped", "done", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list", "--all"])
        .assert()
        .success()
        .stdout("0001 todo active\n0002 done done-but-not-archived\n0003 done shipped\n");
}

#[test]
fn list_short_all_matches_long_all() {
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

    let expected = "0001 todo active\n0002 done shipped\n";
    for flag in ["-a", "--all"] {
        Command::cargo_bin("clew")
            .unwrap()
            .current_dir(temp.path())
            .args(["list", flag])
            .assert()
            .success()
            .stdout(expected);
    }
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
fn list_malformed_frontmatter_errors() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-broken.md",
        "---\nid: 1\nstatus: flying\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\n---\n",
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["list"])
        .assert()
        .failure()
        .code(2)
        .stderr(contains("frontmatter parse error"))
        .stderr(contains("0001-broken.md"));
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

// ---------------------------------------------------------------------------
// `clew start`
// ---------------------------------------------------------------------------

fn read_at(temp: &assert_fs::TempDir, rel: &str) -> String {
    std::fs::read_to_string(temp.path().join(rel)).unwrap()
}

#[test]
fn start_transitions_todo_to_in_progress() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["start", "1"])
        .assert()
        .success()
        .stdout("")
        .stderr(contains("Started #0001"));

    let body = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(body.contains("status: in_progress"));
}

#[test]
fn start_transitions_backlog_to_in_progress() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "backlog", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["start", "0001"])
        .assert()
        .success();

    let body = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(body.contains("status: in_progress"));
}

#[test]
fn start_accepts_slug() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-add-oauth.md",
        &fixture_with(1, "add-oauth", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["start", "add-oauth"])
        .assert()
        .success();
}

#[test]
fn start_in_progress_again_is_invalid_transition() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "in_progress", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["start", "1"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("invalid status transition"))
        .stderr(contains("in_progress"));
}

#[test]
fn start_done_is_invalid_transition() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "done", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["start", "1"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("invalid status transition"));
}

#[test]
fn start_unknown_id_errors() {
    let temp = empty_project();
    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["start", "9999"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("not found"));
}

#[test]
fn start_warns_when_blocked() {
    let temp = empty_project();
    temp.child(".clew/increments/0001-a.md")
        .write_str("---\nid: 1\nstatus: todo\nblocked_reason: \"waiting on #0039\"\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\n---\n# a\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["start", "1"])
        .assert()
        .success()
        .stderr(contains("Started #0001"))
        .stderr(contains("warning"))
        .stderr(contains("blocked"))
        .stderr(contains("waiting on #0039"));

    let body = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(body.contains("status: in_progress"));
    // Serializer canonicalizes `#`-bearing strings with single quotes; both
    // are valid YAML — just assert the value survived.
    assert!(body.contains("blocked_reason: 'waiting on #0039'"));
}

#[test]
fn start_preserves_unknown_fields_and_body() {
    let temp = empty_project();
    let original = "---\nid: 1\nstatus: todo\ntags:\n- auth\n- p0\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\npriority: high\njira: PROJ-1234\n---\n\n# Title\n\n- [x] One\n- [ ] Two\n";
    temp.child(".clew/increments/0001-title.md")
        .write_str(original)
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["start", "1"])
        .assert()
        .success();

    let body = read_at(&temp, ".clew/increments/0001-title.md");
    assert!(body.contains("status: in_progress"));
    assert!(body.contains("priority: high"));
    assert!(body.contains("jira: PROJ-1234"));
    assert!(body.contains("- auth"));
    assert!(body.contains("- p0"));
    assert!(body.contains("# Title"));
    assert!(body.contains("- [x] One"));
    assert!(body.contains("- [ ] Two"));
}

#[test]
fn start_bumps_updated_at() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["start", "1"])
        .assert()
        .success();

    let body = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(body.contains("created_at: 2026-04-20T10:00:00Z"));
    assert!(!body.contains("updated_at: 2026-04-20T10:00:00Z"));
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
        .write_stdin("")
        .assert()
        .success();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["show", "0001"])
        .assert()
        .success()
        .stdout(contains("id: 1"))
        .stdout(contains("# Pipeable"));
}

// ---------------------------------------------------------------------------
// `clew block` / `clew unblock`
// ---------------------------------------------------------------------------

#[test]
fn block_sets_reason_and_bumps_updated_at() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["block", "1", "waiting on #0039"])
        .assert()
        .success()
        .stdout("")
        .stderr(contains("Blocked #0001"));

    let body = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(body.contains("status: todo"));
    assert!(body.contains("blocked_reason: 'waiting on #0039'"));
    assert!(body.contains("created_at: 2026-04-20T10:00:00Z"));
    assert!(!body.contains("updated_at: 2026-04-20T10:00:00Z"));
}

#[test]
fn block_accepts_slug_and_preserves_unknown_fields_and_body() {
    let temp = empty_project();
    let original = "---\nid: 1\nstatus: in_progress\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\npriority: high\n---\n\n# Title\n\n- [ ] One\n";
    temp.child(".clew/increments/0001-title.md")
        .write_str(original)
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["block", "title", "needs design"])
        .assert()
        .success();

    let body = read_at(&temp, ".clew/increments/0001-title.md");
    assert!(body.contains("blocked_reason: needs design"));
    assert!(body.contains("priority: high"));
    assert!(body.contains("# Title"));
    assert!(body.contains("- [ ] One"));
}

#[test]
fn block_rejects_empty_reason() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["block", "1", "   "])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("reason must not be empty"));
}

#[test]
fn block_rejects_terminal_and_archived_increments() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-done-active.md",
        &fixture_with(1, "done-active", "done", None),
    );
    write_increment(
        &temp,
        "archive",
        "0002-archived.md",
        &fixture_with(2, "archived", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["block", "1", "waiting"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("invalid status transition"));

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["block", "2", "waiting"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("cannot block archived increment #0002"));
}

#[test]
fn unblock_removes_reason_and_bumps_updated_at() {
    let temp = empty_project();
    temp.child(".clew/increments/0001-a.md")
        .write_str("---\nid: 1\nstatus: in_progress\nblocked_reason: waiting\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\n---\n# a\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["unblock", "1"])
        .assert()
        .success()
        .stdout("")
        .stderr(contains("Unblocked #0001"));

    let body = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(!body.contains("blocked_reason:"));
    assert!(!body.contains("updated_at: 2026-04-20T10:00:00Z"));
}

#[test]
fn unblock_already_unblocked_warns_without_bumping_timestamp() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["unblock", "a"])
        .assert()
        .success()
        .stdout("")
        .stderr(contains("warning: #0001 is already unblocked"));

    let body = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(body.contains("updated_at: 2026-04-20T10:00:00Z"));
}

// ---------------------------------------------------------------------------
// `clew done`
// ---------------------------------------------------------------------------

#[test]
fn done_transitions_in_progress_to_done_and_archives() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "in_progress", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["done", "1"])
        .assert()
        .success()
        .stdout("")
        .stderr(contains("Done #0001"));

    assert!(!temp.path().join(".clew/increments/0001-a.md").exists());
    let archived = read_at(&temp, ".clew/archive/0001-a.md");
    assert!(archived.contains("status: done"));
}

#[test]
fn done_rejects_backlog() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "backlog", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["done", "1"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("invalid status transition"))
        .stderr(contains("backlog"));

    assert!(temp.path().join(".clew/increments/0001-a.md").exists());
    assert!(!temp.path().join(".clew/archive/0001-a.md").exists());
}

#[test]
fn done_accepts_slug() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-add-oauth.md",
        &fixture_with(1, "add-oauth", "in_progress", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["done", "add-oauth"])
        .assert()
        .success();

    assert!(temp.path().join(".clew/archive/0001-add-oauth.md").exists());
}

#[test]
fn done_removes_increment_from_path_md() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "in_progress", None),
    );
    write_increment(
        &temp,
        "increments",
        "0002-b.md",
        &fixture_with(2, "b", "todo", None),
    );
    temp.child(".clew/path.md")
        .write_str("# Path\n\n- #0001-a\n- #0002-old-b // note\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["done", "1"])
        .assert()
        .success();

    assert_eq!(
        read_at(&temp, ".clew/path.md"),
        "# Path\n\n- #0002-b // note\n"
    );
}

#[test]
fn done_self_loop_tolerates_hand_edited_done_and_archives_without_bumping_timestamp() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "done", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["done", "1"])
        .assert()
        .success()
        .stderr(contains(
            "warning: #0001 already marked done; completing archive",
        ))
        .stderr(contains("Done #0001"));

    let archived = read_at(&temp, ".clew/archive/0001-a.md");
    assert!(archived.contains("status: done"));
    assert!(archived.contains("updated_at: 2026-04-20T10:00:00Z"));
}

#[test]
fn done_already_archived_done_is_success_with_warning() {
    let temp = empty_project();
    write_increment(
        &temp,
        "archive",
        "0001-a.md",
        &fixture_with(1, "a", "done", None),
    );
    temp.child(".clew/path.md")
        .write_str("# Path\n\n- #0001-a\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["done", "1"])
        .assert()
        .success()
        .stderr(contains("warning: #0001 already archived"))
        .stderr(contains("Done #0001"));

    assert!(temp.path().join(".clew/archive/0001-a.md").exists());
    assert_eq!(read_at(&temp, ".clew/path.md"), "# Path\n\n");
}

#[test]
fn done_preserves_unknown_fields_and_body() {
    let temp = empty_project();
    let original = "---\nid: 1\nstatus: in_progress\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\npriority: high\n---\n\n# Title\n\n- [x] One\n";
    temp.child(".clew/increments/0001-title.md")
        .write_str(original)
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["done", "1"])
        .assert()
        .success();

    let body = read_at(&temp, ".clew/archive/0001-title.md");
    assert!(body.contains("status: done"));
    assert!(body.contains("priority: high"));
    assert!(body.contains("# Title"));
    assert!(body.contains("- [x] One"));
}

// ---------------------------------------------------------------------------
// `clew abandon`
// ---------------------------------------------------------------------------

#[test]
fn abandon_transitions_in_progress_to_abandoned_archives_and_records_reason() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "in_progress", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["abandon", "1", "not worth doing"])
        .assert()
        .success()
        .stdout("")
        .stderr(contains("Abandoned #0001"));

    assert!(!temp.path().join(".clew/increments/0001-a.md").exists());
    let archived = read_at(&temp, ".clew/archive/0001-a.md");
    assert!(archived.contains("status: abandoned"));
    assert!(archived.contains("abandoned_reason: not worth doing"));
}

#[test]
fn abandon_accepts_slug_and_removes_increment_from_path_md() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-add-oauth.md",
        &fixture_with(1, "add-oauth", "todo", None),
    );
    write_increment(
        &temp,
        "increments",
        "0002-b.md",
        &fixture_with(2, "b", "todo", None),
    );
    temp.child(".clew/path.md")
        .write_str("# Path\n\n- #0001-add-oauth\n- #0002-old-b // note\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["abandon", "add-oauth", "superseded"])
        .assert()
        .success();

    assert!(temp.path().join(".clew/archive/0001-add-oauth.md").exists());
    assert_eq!(
        read_at(&temp, ".clew/path.md"),
        "# Path\n\n- #0002-b // note\n"
    );
}

#[test]
fn abandon_transitions_done_to_abandoned() {
    let temp = empty_project();
    write_increment(
        &temp,
        "archive",
        "0001-a.md",
        &fixture_with(1, "a", "done", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["abandon", "1", "shipped wrong thing"])
        .assert()
        .success()
        .stderr(contains("Abandoned #0001"));

    let archived = read_at(&temp, ".clew/archive/0001-a.md");
    assert!(archived.contains("status: abandoned"));
    assert!(archived.contains("abandoned_reason: shipped wrong thing"));
}

#[test]
fn abandon_without_reason_warns_and_archives() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["abandon", "1"])
        .assert()
        .success()
        .stdout("")
        .stderr(contains(
            "warning: #0001 is abandoned without an abandoned_reason",
        ))
        .stderr(contains("Abandoned #0001"));

    let archived = read_at(&temp, ".clew/archive/0001-a.md");
    assert!(archived.contains("status: abandoned"));
    assert!(!archived.contains("abandoned_reason:"));
}

#[test]
fn abandon_with_whitespace_reason_warns_and_archives_without_reason() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["abandon", "1", "   "])
        .assert()
        .success()
        .stderr(contains(
            "warning: #0001 is abandoned without an abandoned_reason",
        ));

    let archived = read_at(&temp, ".clew/archive/0001-a.md");
    assert!(archived.contains("status: abandoned"));
    assert!(!archived.contains("abandoned_reason:"));
}

#[test]
fn abandon_self_loop_tolerates_hand_edited_abandoned_and_archives() {
    let temp = empty_project();
    temp.child(".clew/increments/0001-a.md")
        .write_str("---\nid: 1\nstatus: abandoned\nabandoned_reason: already decided\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\n---\n#a\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["abandon", "1", "not used on self-loop"])
        .assert()
        .success()
        .stderr(contains(
            "warning: #0001 already marked abandoned; completing archive",
        ))
        .stderr(contains("Abandoned #0001"));

    let archived = read_at(&temp, ".clew/archive/0001-a.md");
    assert!(archived.contains("status: abandoned"));
    assert!(archived.contains("abandoned_reason: already decided"));
    assert!(archived.contains("updated_at: 2026-04-20T10:00:00Z"));
}

#[test]
fn abandon_self_loop_warns_when_hand_edited_without_reason() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "abandoned", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["abandon", "1", "too late"])
        .assert()
        .success()
        .stderr(contains(
            "warning: #0001 is abandoned without an abandoned_reason",
        ))
        .stderr(contains("Abandoned #0001"));

    let archived = read_at(&temp, ".clew/archive/0001-a.md");
    assert!(archived.contains("status: abandoned"));
    assert!(!archived.contains("abandoned_reason:"));
}

#[test]
fn abandon_already_archived_abandoned_is_success_with_warning() {
    let temp = empty_project();
    write_increment(
        &temp,
        "archive",
        "0001-a.md",
        &fixture_with(1, "a", "abandoned", None),
    );
    temp.child(".clew/path.md")
        .write_str("# Path\n\n- #0001-a\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["abandon", "1", "already gone"])
        .assert()
        .success()
        .stderr(contains("warning: #0001 already archived"))
        .stderr(contains("Abandoned #0001"));

    assert!(temp.path().join(".clew/archive/0001-a.md").exists());
    assert_eq!(read_at(&temp, ".clew/path.md"), "# Path\n\n");
}

#[test]
fn abandon_preserves_unknown_fields_and_body() {
    let temp = empty_project();
    let original = "---\nid: 1\nstatus: backlog\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\npriority: high\n---\n\n# Title\n\n- [ ] One\n";
    temp.child(".clew/increments/0001-title.md")
        .write_str(original)
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["abandon", "1", "won't do"])
        .assert()
        .success();

    let body = read_at(&temp, ".clew/archive/0001-title.md");
    assert!(body.contains("status: abandoned"));
    assert!(body.contains("abandoned_reason: won't do"));
    assert!(body.contains("priority: high"));
    assert!(body.contains("# Title"));
    assert!(body.contains("- [ ] One"));
}

// ---------------------------------------------------------------------------
// `clew reopen`
// ---------------------------------------------------------------------------

#[test]
fn reopen_transitions_archived_done_to_todo_and_unarchives() {
    let temp = empty_project();
    write_increment(
        &temp,
        "archive",
        "0001-a.md",
        &fixture_with(1, "a", "done", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["reopen", "1"])
        .assert()
        .success()
        .stdout("")
        .stderr(contains("Reopened #0001"));

    assert!(!temp.path().join(".clew/archive/0001-a.md").exists());
    let reopened = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(reopened.contains("status: todo"));
    assert!(!reopened.contains("updated_at: 2026-04-20T10:00:00Z"));
}

#[test]
fn reopen_accepts_slug_for_archived_abandoned_and_preserves_reason() {
    let temp = empty_project();
    temp.child(".clew/archive/0001-add-oauth.md")
        .write_str("---\nid: 1\nstatus: abandoned\nabandoned_reason: superseded\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\n---\n# Add OAuth\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["reopen", "add-oauth"])
        .assert()
        .success()
        .stderr(contains("Reopened #0001"));

    assert!(!temp.path().join(".clew/archive/0001-add-oauth.md").exists());
    let reopened = read_at(&temp, ".clew/increments/0001-add-oauth.md");
    assert!(reopened.contains("status: todo"));
    assert!(reopened.contains("abandoned_reason: superseded"));
}

#[test]
fn reopen_self_loop_tolerates_unarchived_todo_without_bumping_timestamp() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["reopen", "1"])
        .assert()
        .success()
        .stderr(contains("warning: #0001 already reopened"))
        .stderr(contains("Reopened #0001"));

    let reopened = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(reopened.contains("status: todo"));
    assert!(reopened.contains("updated_at: 2026-04-20T10:00:00Z"));
}

#[test]
fn reopen_self_loop_tolerates_archived_todo_and_unarchives_without_bumping_timestamp() {
    let temp = empty_project();
    write_increment(
        &temp,
        "archive",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["reopen", "1"])
        .assert()
        .success()
        .stderr(contains(
            "warning: #0001 already marked todo; completing unarchive",
        ))
        .stderr(contains("Reopened #0001"));

    assert!(!temp.path().join(".clew/archive/0001-a.md").exists());
    let reopened = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(reopened.contains("status: todo"));
    assert!(reopened.contains("updated_at: 2026-04-20T10:00:00Z"));
}

#[test]
fn reopen_tolerates_unarchived_done_drift() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "done", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["reopen", "1"])
        .assert()
        .success()
        .stderr(contains("Reopened #0001"));

    let reopened = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(reopened.contains("status: todo"));
    assert!(!reopened.contains("updated_at: 2026-04-20T10:00:00Z"));
}

#[test]
fn reopen_tolerates_unarchived_abandoned_drift() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "abandoned", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["reopen", "1"])
        .assert()
        .success()
        .stderr(contains("Reopened #0001"));

    let reopened = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(reopened.contains("status: todo"));
    assert!(!reopened.contains("updated_at: 2026-04-20T10:00:00Z"));
}

#[test]
fn reopen_rejects_backlog() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "backlog", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["reopen", "1"])
        .assert()
        .failure()
        .code(1)
        .stderr(contains("invalid status transition"))
        .stderr(contains("backlog"));

    assert!(temp.path().join(".clew/increments/0001-a.md").exists());
}

#[test]
fn reopen_preserves_unknown_fields_and_body() {
    let temp = empty_project();
    let original = "---\nid: 1\nstatus: done\ncreated_at: 2026-04-20T10:00:00Z\nupdated_at: 2026-04-20T10:00:00Z\npriority: high\n---\n\n# Title\n\n- [x] One\n";
    temp.child(".clew/archive/0001-title.md")
        .write_str(original)
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["reopen", "1"])
        .assert()
        .success();

    let body = read_at(&temp, ".clew/increments/0001-title.md");
    assert!(body.contains("status: todo"));
    assert!(body.contains("priority: high"));
    assert!(body.contains("# Title"));
    assert!(body.contains("- [x] One"));
}

// ---------------------------------------------------------------------------
// `clew next`
// ---------------------------------------------------------------------------

fn fixture_with_created_at(id: u32, slug: &str, status: &str, created_at: &str) -> String {
    format!(
        "---\nid: {id}\nstatus: {status}\ncreated_at: {created_at}\nupdated_at: 2026-04-20T10:00:00Z\n---\n# {slug}\n"
    )
}

#[test]
fn next_returns_first_path_reference() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-oldest.md",
        &fixture_with_created_at(1, "oldest", "todo", "2026-04-20T10:00:00Z"),
    );
    write_increment(
        &temp,
        "increments",
        "0002-priority.md",
        &fixture_with_created_at(2, "priority", "todo", "2026-04-21T10:00:00Z"),
    );
    temp.child(".clew/path.md")
        .write_str("# Path\n\nnotes are ignored\n- #0002-priority\n- #0001-oldest\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("next")
        .assert()
        .success()
        .stdout("0002\n")
        .stderr("");
}

#[test]
fn next_falls_back_to_oldest_todo_when_path_is_empty() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-backlog.md",
        &fixture_with_created_at(1, "backlog", "backlog", "2026-04-19T10:00:00Z"),
    );
    write_increment(
        &temp,
        "increments",
        "0002-newer.md",
        &fixture_with_created_at(2, "newer", "todo", "2026-04-21T10:00:00Z"),
    );
    write_increment(
        &temp,
        "increments",
        "0003-older.md",
        &fixture_with_created_at(3, "older", "todo", "2026-04-20T10:00:00Z"),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("next")
        .assert()
        .success()
        .stdout("0003\n");
}

#[test]
fn next_start_marks_selected_increment_in_progress() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with_created_at(1, "a", "todo", "2026-04-20T10:00:00Z"),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .args(["next", "--start"])
        .assert()
        .success()
        .stdout("0001\n")
        .stderr(contains("Started #0001"));

    let body = read_at(&temp, ".clew/increments/0001-a.md");
    assert!(body.contains("status: in_progress"));
}

#[test]
fn next_errors_when_no_todo_exists() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-backlog.md",
        &fixture_with_created_at(1, "backlog", "backlog", "2026-04-20T10:00:00Z"),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("next")
        .assert()
        .failure()
        .code(1)
        .stderr(contains("no todo increments found"));
}

#[test]
fn next_errors_on_stale_path_reference() {
    let temp = empty_project();
    temp.child(".clew/path.md")
        .write_str("# Path\n\n- #0009-missing\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("next")
        .assert()
        .failure()
        .code(1)
        .stderr(contains("increment not found: 9"));
}

// ---------------------------------------------------------------------------
// `clew lint`
// ---------------------------------------------------------------------------

#[test]
fn lint_succeeds_when_project_has_no_drift() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-a.md",
        &fixture_with(1, "a", "todo", None),
    );
    temp.child(".clew/path.md")
        .write_str("# Path\n\n- #0001-a\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("lint")
        .assert()
        .success()
        .stderr(contains("No lint issues found"));
}

#[test]
fn lint_flags_path_references_that_cannot_be_selected() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-in-progress.md",
        &fixture_with(1, "in-progress", "in_progress", None),
    );
    write_increment(
        &temp,
        "archive",
        "0002-done.md",
        &fixture_with(2, "done", "done", None),
    );
    temp.child(".clew/path.md")
        .write_str("# Path\n\n- #0001-in-progress\n- #0002-done\n- #0009-missing\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("lint")
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "path.md references #0001-in-progress with status in_progress; expected todo",
        ))
        .stderr(contains("path.md references archived #0002-done"))
        .stderr(contains("path.md references missing #0009"))
        .stderr(contains("error: lint found 3 issue(s)"));
}

#[test]
fn lint_flags_terminal_statuses_left_in_increments() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-finished.md",
        &fixture_with(1, "finished", "done", None),
    );
    write_increment(
        &temp,
        "increments",
        "0002-dropped.md",
        &fixture_with(2, "dropped", "abandoned", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("lint")
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "#0001-finished has status done but is not archived; run `clew done 0001`",
        ))
        .stderr(contains(
            "#0002-dropped has status abandoned but is not archived; run `clew abandon 0002`",
        ));
}

#[test]
fn lint_flags_todo_items_missing_from_non_empty_path() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-listed.md",
        &fixture_with(1, "listed", "todo", None),
    );
    write_increment(
        &temp,
        "increments",
        "0002-missing.md",
        &fixture_with(2, "missing", "todo", None),
    );
    temp.child(".clew/path.md")
        .write_str("# Path\n\n- #0001-listed\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("lint")
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "#0002-missing is todo but missing from path.md priority order",
        ));
}

#[test]
fn lint_flags_archived_non_terminal_without_bad_reopen_hint() {
    let temp = empty_project();
    write_increment(
        &temp,
        "archive",
        "0001-not-terminal.md",
        &fixture_with(1, "not-terminal", "backlog", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("lint")
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "#0001-not-terminal is archived with status backlog",
        ))
        .stderr(contains(
            "move it back to .clew/increments/ or change status to done/abandoned",
        ));
}

#[test]
fn lint_flags_filename_frontmatter_id_mismatch() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-wrong-id.md",
        &fixture_with(2, "wrong-id", "todo", None),
    );

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("lint")
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "filename #0001-wrong-id has frontmatter id 2; make them match before running transition commands",
        ));
}

#[test]
fn lint_flags_stale_path_reference_per_line() {
    let temp = empty_project();
    write_increment(
        &temp,
        "increments",
        "0001-current.md",
        &fixture_with(1, "current", "todo", None),
    );
    temp.child(".clew/path.md")
        .write_str("# Path\n\n- #0001-old\n<!-- canonical elsewhere #0001-current -->\n")
        .unwrap();

    Command::cargo_bin("clew")
        .unwrap()
        .current_dir(temp.path())
        .arg("lint")
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "path.md reference #0001 is not canonical; expected #0001-current",
        ));
}
