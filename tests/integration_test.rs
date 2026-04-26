use assert_cmd::Command;

#[test]
fn version_flag_works() {
    let mut cmd = Command::cargo_bin("clew").unwrap();
    cmd.arg("--version").assert().success();
}
