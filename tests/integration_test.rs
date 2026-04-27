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
