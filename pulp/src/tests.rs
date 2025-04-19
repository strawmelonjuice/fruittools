#[test]
pub(crate) fn test_detect_dev_tool_gleam_plus() {
    use crate::toolchains::{Target, ToolChainName, ToolNameGleam};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    let test_dir =
        PathBuf::from("./test_toolchain_detection/gleam/lustre_dev_tools_and_gleescript");
    fs::create_dir_all(&test_dir).unwrap();

    // Create a test gleam.toml file
    let mut file = fs::File::create(test_dir.join("gleam.toml")).unwrap();
    let gleam_toml_content = r#"# Taken from https://github.com/lustre-labs/lustre/blob/main/examples/01-basics/01-hello-world/gleam.toml :)
name = "app"
version = "1.0.0"
# Lustre-dev-tools should be ignored for a build run, but preffered in a dev run.
dev-dependencies = { lustre_dev_tools = ">= 1.6.5 and < 2.0.0" }

[dependencies]
gleam_stdlib = ">= 0.44.0 and < 2.0.0"
# Gleescript
gleescript = ">= 1.2.0 and < 2.0.0""#;
    write!(file, "{}", gleam_toml_content).unwrap();

    // Test detection for target:dev, should detect lustre_dev_tools
    let result = ToolChainName::detect(test_dir.clone(), Target::Dev);
    // Comparing to a Result::Ok(blabla) failed, because the Error type does not implement PartialEq.
    // So we need to unwrap the result and compare the inner value instead, no biggie.
    let (_certainty, result) =
        result.expect("Failed to detect toolchain, so this test definitely failed.");
    assert_eq!(
        result,
        ToolChainName::Gleam(ToolNameGleam::LustreDev),
        "Target=Dev, so, should detect lustre_dev_tools as the toolchain, but it didn't."
    );

    // Test detection for target:build, should detect gleescript
    let result = ToolChainName::detect(test_dir.clone(), Target::Build);
    // Comparing to a Result::Ok(blabla) failed, because the Error type does not implement PartialEq.
    // So we need to unwrap the result and compare the inner value instead, no biggie.
    let (_certainty, result) =
        result.expect("Failed to detect toolchain, so this test definitely failed.");
    assert_eq!(
        result,
        ToolChainName::Gleam(ToolNameGleam::GleEscript),
        "Target=Build, so, should detect gleescript as the toolchain, but it didn't."
    );

    // Clean up
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
pub(crate) fn test_detect_dev_tool_gleam() {
    use crate::toolchains::{Target, ToolChainName, ToolNameGleam};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    let test_dir = PathBuf::from("./test_toolchain_detection/gleam/gleam");
    fs::create_dir_all(&test_dir).unwrap();
    // Create a test gleam.toml file
    let mut file = fs::File::create(test_dir.join("gleam.toml")).unwrap();
    let content = r#"name = "app"
version = "1.0.0"

# Fill out these fields if you intend to generate HTML documentation or publish
# your project to the Hex package manager.
#
# description = ""
# licences = ["Apache-2.0"]
# repository = { type = "github", user = "", repo = "" }
# links = [{ title = "Website", href = "" }]
#
# For a full reference of all the available options, you can have a look at
# https://gleam.run/writing-gleam/gleam-toml/.

[dependencies]
gleam_stdlib = ">= 0.44.0 and < 2.0.0"

[dev-dependencies]
gleeunit = ">= 1.0.0 and < 2.0.0"
"#;
    write!(file, "{}", content).unwrap();

    // Test detection for target:dev, should detect rust
    let result = ToolChainName::detect(test_dir.clone(), Target::Dev);
    // Comparing to a Result::Ok(blabla) failed, because the Error type does not implement PartialEq.
    // So we need to unwrap the result and compare the inner value instead, no biggie.
    let (_certainty, result) =
        result.expect("Failed to detect toolchain, so this test definitely failed.");
    assert_eq!(
        result,
        ToolChainName::Gleam(ToolNameGleam::None),
        "Target=Dev, so, should detect gleam as the toolchain, but it didn't."
    );

    // Clean up
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
pub(crate) fn test_detect_dev_tool_rust() {
    use crate::toolchains::{Target, ToolChainName, ToolNameRust};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    let test_dir = PathBuf::from("./test_toolchain_detection/rust");
    fs::create_dir_all(&test_dir.join("src")).unwrap();

    // Create a test Cargo.toml file
    let mut file = fs::File::create(test_dir.join("Cargo.toml")).unwrap();
    let cargo_toml_content = r#"[package]
name = "app"
version = "1.0.0"
edition = "2021"
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
# Add your dependencies here
"#;
    write!(file, "{}", cargo_toml_content).unwrap();
    // Create a test src/main.rs file
    let mut file = fs::File::create(test_dir.join("src/main.rs")).unwrap();
    let main_rs_content = r#"fn main() {
    println!("Hello, world!");
}
"#;
    write!(file, "{}", main_rs_content).unwrap();

    // Test detection for target:dev, should detect rust
    let result = ToolChainName::detect(test_dir.clone(), Target::Dev);
    // Comparing to a Result::Ok(blabla) failed, because the Error type does not implement PartialEq.
    // So we need to unwrap the result and compare the inner value instead, no biggie.
    let (_certainty, result) =
        result.expect("Failed to detect toolchain, so this test definitely failed.");
    assert_eq!(
        result,
        ToolChainName::Rust(ToolNameRust::None),
        "Target=Dev, so, should detect rust as the toolchain, but it didn't."
    );

    // Create a test src/tests.rs file
    let mut file = fs::File::create(test_dir.join("src/tests.rs")).unwrap();
    let tests_rs_content = r#"#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
"#;
    write!(file, "{}", tests_rs_content).unwrap();
    // Test detection for target:dev, should detect rust tests
    let result = ToolChainName::detect(test_dir.clone(), Target::Dev);
    let (_certainty, result) =
        result.expect("Failed to detect toolchain, so this test definitely failed.");
    assert_eq!(
        result,
        ToolChainName::Rust(ToolNameRust::Tests),
        "Target=Dev, so, should detect rust with tests as the toolchain, but it didn't."
    );

    // Clean up
    fs::remove_dir_all(test_dir).unwrap();
}
