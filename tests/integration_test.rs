use assert_cmd::Command;
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
