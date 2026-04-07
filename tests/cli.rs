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
fn test_length_too_long() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--length").arg("129");
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
fn test_zero_count_csv_format() {
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("0").arg("--format").arg("csv");
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

// --- Tests for PR changes: MAX_LENGTH validation and length boundary behavior ---

#[test]
fn test_length_minimum_valid() {
    // length=2 is the minimum allowed value; should succeed and output a 2-char ID
    let output = Command::cargo_bin("cuid2gen")
        .unwrap()
        .arg("--length")
        .arg("2")
        .output()
        .unwrap();
    assert!(output.status.success());
    let id = String::from_utf8(output.stdout).unwrap();
    let id = id.trim();
    assert_eq!(id.len(), 2, "ID length should be 2, got: {}", id.len());
    assert!(
        id.chars()
            .all(|c| c.is_ascii_digit() || (c.is_ascii_alphabetic() && c.is_lowercase())),
        "ID should contain only lowercase alphanumeric characters"
    );
}

#[test]
fn test_length_zero_rejected() {
    // length=0 is below the minimum of 2 and must be rejected
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--length").arg("0");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Length must be at least 2"));
}

#[test]
fn test_length_one_rejected() {
    // length=1 is below the minimum of 2 and must be rejected
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--length").arg("1");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Length must be at least 2"));
}

#[test]
fn test_length_maximum_valid() {
    // length=128 (MAX_LENGTH) is the maximum allowed value; should succeed
    let output = Command::cargo_bin("cuid2gen")
        .unwrap()
        .arg("--length")
        .arg("128")
        .output()
        .unwrap();
    assert!(output.status.success());
    let id = String::from_utf8(output.stdout).unwrap();
    let id = id.trim();
    assert_eq!(id.len(), 128, "ID length should be 128, got: {}", id.len());
}

#[test]
fn test_length_exceeds_max() {
    // length=129 exceeds MAX_LENGTH=128 and must be rejected
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--length").arg("129");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Length exceeds maximum allowed value of 128"));
}

#[test]
fn test_length_with_multiple_ids() {
    // count=3 combined with length=6 should produce 3 lines each with exactly 6-char IDs
    let output = Command::cargo_bin("cuid2gen")
        .unwrap()
        .arg("--count")
        .arg("3")
        .arg("--length")
        .arg("6")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 3, "Should output exactly 3 lines");
    for line in lines {
        assert_eq!(line.len(), 6, "Each ID should be 6 chars, got: {}", line.len());
        assert!(
            line.chars().all(|c| c.is_ascii_alphanumeric() && (c.is_ascii_digit() || c.is_lowercase())),
            "ID should be lowercase alphanumeric: {}", line
        );
    }
}

// --- Tests for JSON format changes (now writes {"ids":[...]} manually) ---

#[test]
fn test_json_format_multiple_ids() {
    // count=3 with json format must produce {"ids":["x","y","z"]} with 3 elements
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("3").arg("--format").arg("json");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r#"^\{"ids":\["[a-z0-9]+","[a-z0-9]+","[a-z0-9]+"\]\}\n$"#).unwrap());
}

#[test]
fn test_json_format_ids_key() {
    // JSON output must use an "ids" key containing an array (not a bare array)
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("2").arg("--format").arg("json");
    // Verify the output starts with {"ids":[ and ends with ]} — confirming the ids-keyed object structure
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with(r#"{"ids":["#))
        .stdout(predicate::str::contains(r#"","#))  // comma between two IDs
        .stdout(predicate::str::ends_with("]}\n"));
}

#[test]
fn test_json_format_with_length() {
    // length=8 + json format: each id in the "ids" array must be exactly 8 chars
    let output = Command::cargo_bin("cuid2gen")
        .unwrap()
        .arg("--count")
        .arg("2")
        .arg("--length")
        .arg("8")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    // Parse out the ids manually: strip {"ids":[ prefix and ]} suffix
    let inner = text
        .trim()
        .trim_start_matches(r#"{"ids":["#)
        .trim_end_matches("]}");
    // Split by `","` to get individual quoted ids
    let raw_ids: Vec<&str> = inner.split("\",\"").collect();
    assert_eq!(raw_ids.len(), 2, "Should have 2 ids in the JSON array");
    for raw in raw_ids {
        // Strip surrounding quotes that may remain on first/last element
        let id = raw.trim_matches('"');
        assert_eq!(id.len(), 8, "Each JSON id should be 8 chars, got: {}", id.len());
        assert!(
            id.chars().all(|c| c.is_ascii_alphanumeric() && (c.is_ascii_digit() || c.is_lowercase())),
            "JSON id should be lowercase alphanumeric: {}", id
        );
    }
}

// --- Tests for CSV format changes (now writes incrementally) ---

#[test]
fn test_csv_format_single_id() {
    // count=1 with csv format must output a single ID with no commas
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("1").arg("--format").arg("csv");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^[a-z0-9]+\n$").unwrap());
}

#[test]
fn test_csv_format_three_ids() {
    // count=3 with csv format must output exactly two commas separating three IDs
    let output = Command::cargo_bin("cuid2gen")
        .unwrap()
        .arg("--count")
        .arg("3")
        .arg("--format")
        .arg("csv")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    let line = text.trim();
    let parts: Vec<&str> = line.split(',').collect();
    assert_eq!(parts.len(), 3, "CSV output should have 3 comma-separated IDs");
    for part in parts {
        assert!(!part.is_empty(), "CSV part should not be empty");
        assert!(
            part.chars().all(|c| c.is_ascii_alphanumeric() && (c.is_ascii_digit() || c.is_lowercase())),
            "CSV ID should be lowercase alphanumeric: {}", part
        );
    }
}

#[test]
fn test_csv_format_with_length() {
    // count=3 + length=7 + csv: each comma-separated field should be exactly 7 chars
    let output = Command::cargo_bin("cuid2gen")
        .unwrap()
        .arg("--count")
        .arg("3")
        .arg("--length")
        .arg("7")
        .arg("--format")
        .arg("csv")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    let line = text.trim();
    let parts: Vec<&str> = line.split(',').collect();
    assert_eq!(parts.len(), 3);
    for part in parts {
        assert_eq!(part.len(), 7, "CSV ID should be 7 chars, got: {}", part.len());
    }
}

// --- Tests for quiet mode with different formats ---

#[test]
fn test_quiet_with_json_format() {
    // quiet mode must suppress json output
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--quiet").arg("--format").arg("json");
    cmd.assert().success().stdout(predicate::str::is_empty());
}

#[test]
fn test_quiet_with_csv_format() {
    // quiet mode must suppress csv output even with multiple IDs
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--quiet").arg("--count").arg("3").arg("--format").arg("csv");
    cmd.assert().success().stdout(predicate::str::is_empty());
}

#[test]
fn test_quiet_with_count_and_length() {
    // quiet mode must suppress output regardless of count and length arguments
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--quiet").arg("--count").arg("5").arg("--length").arg("10");
    cmd.assert().success().stdout(predicate::str::is_empty());
}

// --- Error message validation for count exceeds max ---

#[test]
fn test_count_exceeds_max_error_message() {
    // count > MAX_COUNT must fail with a descriptive error message on stderr
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("1000001");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Count exceeds maximum allowed value"));
}

// --- Regression: zero count produces no output across all formats ---

#[test]
fn test_zero_count_json_format() {
    // count=0 with json format should succeed and output empty ids array
    let output = Command::cargo_bin("cuid2gen")
        .unwrap()
        .arg("--count")
        .arg("0")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    // With count=0, the JSON loop never runs; output should be {"ids":[]}\n
    assert_eq!(text.trim(), r#"{"ids":[]}"#, "count=0 json should yield empty ids array");
}

#[test]
fn test_zero_count_csv_format() {
    // count=0 with csv format should succeed and produce no output (no IDs to join)
    let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
    cmd.arg("--count").arg("0").arg("--format").arg("csv");
    // The csv branch writes a trailing newline after the loop; with 0 IDs only the newline is written
    cmd.assert().success();
}