use assert_cmd::Command;

#[test]
fn output() {
    let mut cmd = Command::cargo_bin("struct-impl").unwrap();
    let assert = cmd.assert();
    assert.success().stdout("Jack is 34 years old");
}
