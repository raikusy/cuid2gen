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
fn test_custom_length() {
    let output = Command::cargo_bin("cuid2gen")
        .unwrap()
        .arg("--length")
        .arg("10")
        .output()
        .unwrap();
    assert!(output.status.success());
    let id = String::from_utf8(output.stdout).unwrap();
    let id = id.trim();
    assert_eq!(id.len(), 10, "ID length should be 10, got: {}", id.len());
    assert!(
        id.chars()
            .all(|c| c.is_ascii_digit() || (c.is_ascii_alphabetic() && c.is_lowercase())),
        "ID should contain only lowercase alphanumeric characters"
    );
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

#[test]
fn test_length_too_short() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--length").arg("1");
    cmd.assert().failure();
}

#[test]
fn test_count_exceeds_max() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("1000001");
    cmd.assert().failure();
}

#[test]
fn test_zero_count() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("0");
    cmd.assert().success().stdout(predicate::str::is_empty());
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("cuid2gen"));
}
