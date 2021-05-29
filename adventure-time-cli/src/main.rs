use assert_cmd::Command;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

// read in the manifest at build-time
const BUILTIN_MANIFEST_STR: &str = include_str!("../../manifest.yaml");

fn main() {
    let manifest = get_manifest();
    let failing_crate = manifest.order.iter().find(|crate_identifier| {
        let mut cmd = Command::new("cargo");
        let result = cmd.args(&["test", "--quiet", "-p", crate_identifier]).ok();
        match result {
            Ok(_) => {
                println!("{} {}", "✓".green(), crate_identifier);
                false
            }
            Err(_) => {
                println!("{} {}", "⚠".yellow(), crate_identifier);
                true
            }
        }
    });
    match failing_crate {
        Some(crate_name) => {
            println!("\nThe tests for `{}` did not pass run the following cargo command to see the test output for this crate.

{} {}",crate_name.yellow(), "cargo test -p".blue(), crate_name.blue());
        }
        None => {
            println!("\n All tests passed! {} You've completed the beginning of your Rust Adventure and are ready to move on to other projects!", "Congratulations!".green())
        }
    }
}

fn get_manifest() -> Manifest {
    serde_yaml::from_str(BUILTIN_MANIFEST_STR).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    order: Vec<String>,
}
