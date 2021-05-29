use assert_cmd::Command;

#[test]
fn output() {
    let mut cmd = Command::cargo_bin("result").unwrap();
    let assert = cmd.assert();
    assert.success().stdout("655");
}
