use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_basic_generation() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^[a-z0-9]+\n$").unwrap());
}

#[test]
fn test_multiple_ids() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("3");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^[a-z0-9]+\n[a-z0-9]+\n[a-z0-9]+\n$").unwrap());
}

#[test]
fn test_length_warning() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--length").arg("10");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("not supported"));
}

#[test]
fn test_json_format() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--format").arg("json");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r#"^\{"ids":\["[a-z0-9]+"\]\}\n$"#).unwrap());
}

#[test]
fn test_csv_format() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("2").arg("--format").arg("csv");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^[a-z0-9]+,[a-z0-9]+\n$").unwrap());
}

#[test]
fn test_quiet_mode() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--quiet");
    cmd.assert().success().stdout(predicate::str::is_empty());
}
