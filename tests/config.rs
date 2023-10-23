use std::{process::Command, env::temp_dir, fs::canonicalize};

use assert_cmd::prelude::*;
use assert_fs::prelude::*;

const FULL_CONFIG: &str = r#"
{
    "url": "https://example.com",
    "title": "My website",
    "description": "A description of the website"
}
"#;


#[test]
fn locates_and_loads_specified_config_file() -> anyhow::Result<()> {
    let testdir = assert_fs::TempDir::new()?;
    let config_file = testdir.child("shakyrave.json");
    config_file.write_str(FULL_CONFIG)?;

    let mut cmd = Command::cargo_bin("shakyrave")?;
    cmd.current_dir(canonicalize(testdir.path())?)
       .arg("--config")
       .arg("shakyrave.json")
       .assert()
       .success();

    testdir.close()?;

    Ok(())
}
