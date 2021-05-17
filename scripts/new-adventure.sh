#!/bin/bash
set -euxo pipefail

cargo new adventures/$1 --name $1
cargo add -p $1 assert_cmd
mkdir -p adventures/$1/tests
echo "use assert_cmd::Command;

#[test]
fn output() {
    let mut cmd = Command::cargo_bin(\"$1\").unwrap();
    let assert = cmd.assert();
    assert.success().stdout(\"655\");
}" > adventures/$1/tests/output.rs

echo $1 >> manifest.txt