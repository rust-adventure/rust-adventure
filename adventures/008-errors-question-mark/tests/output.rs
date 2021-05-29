use assert_cmd::Command;

#[test]
fn output() {
    let mut cmd = Command::cargo_bin("errors-question-mark").unwrap();
    let assert = cmd.assert();
    assert.success().stdout("10");
}
