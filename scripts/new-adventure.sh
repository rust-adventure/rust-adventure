#!/bin/bash
set -euxo pipefail

LAST_FILE=`fd ^a- 'adventures/' -t d -d 1 | sort | tail -n 1`

regex="^adventures/a-([[:digit:]]+)"
if [[ $LAST_FILE =~ $regex ]]; then
  number="${BASH_REMATCH[1]}"
  number=$((number+1))
  formatted_number=$(printf "%03d" $number)
  cargo new adventures/a-$formatted_number-$1 --name $1
  cargo add -p $1 assert_cmd
  mkdir -p adventures/a-$formatted_number-$1/tests
  echo "use assert_cmd::Command;

#[test]
fn output() {
    let mut cmd = Command::cargo_bin(\"$1\").unwrap();
    let assert = cmd.assert();
    assert.success().stdout(\"655\");
}" > adventures/a-$formatted_number-$1/tests/output.rs
else
  echo "$LAST_FILE doesn't match $regex"
fi