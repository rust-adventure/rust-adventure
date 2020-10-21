# Rust Adventure

## Running an exercise

There are two kinds of exercise. One is a binary and the other is a library.

The binary exercises can be run with `cargo`

```shell
cargo run --bin hello-world
```

## Running Tests

Both kinds of exercises come with tests. You can run the tests for a specific exercise using cargo.

```shell
cargo test -p hello-world
```

Some exercises have more than one test. Giving the prefix of a test you want to run will scope down the tests to the ones that match that prefix.

```shell
cargo test -p hello-world output
```

<details>
<summary>sample output</summary>

```````shell
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running /Users/chris/github/christopherbiscardi/rust-adventure/target/debug/deps/hello_world-5edb99e9dd6fe5cd

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running /Users/chris/github/christopherbiscardi/rust-adventure/target/debug/deps/output-1191747d69332647

running 1 test
test hello_world_output ... FAILED

failures:

---- hello_world_output stdout ----
thread 'hello_world_output' panicked at 'Unexpected stdout, failed var - original <= 0
├── original: Hello, World!
├── actual distance: 2
├── diff: Hello, World!
Welcome to Rust!

└── var as str: Welcome to Rust!

command=`"/Users/chris/github/christopherbiscardi/rust-adventure/target/debug/hello-world"`
code=0
stdout=```Welcome to Rust!```
stderr=``````
', /Users/chris/.cargo/registry/src/github.com-1ecc6299db9ec823/assert_cmd-1.0.1/src/assert.rs:342:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    hello_world_output

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '-p hello-world --test output'
```````

</details>
