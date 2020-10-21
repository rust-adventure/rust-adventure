use assert_cmd::Command;

#[test]
fn test_output() {
    let mut cmd = Command::cargo_bin("hello-world").unwrap();
    let assert = cmd.assert();
    assert.success().stdout("Hello, World!");
}
