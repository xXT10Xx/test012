use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("rcli").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("An advanced CLI tool"));
}

#[test]
fn test_config_init() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.yaml");

    let mut cmd = Command::cargo_bin("rcli").unwrap();
    cmd.args(&["config", "init", "--output", config_path.to_str().unwrap()]);
    cmd.assert().success();

    assert!(config_path.exists());
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("server:"));
    assert!(content.contains("logging:"));
    assert!(content.contains("storage:"));
}

#[test]
fn test_config_show() {
    let mut cmd = Command::cargo_bin("rcli").unwrap();
    cmd.args(&["config", "show"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("server:"))
        .stdout(predicate::str::contains("logging:"))
        .stdout(predicate::str::contains("storage:"));
}

#[test]
fn test_store_and_get() {
    let temp_dir = TempDir::new().unwrap();
    let data_dir = temp_dir.path().join("data");
    
    let test_data = r#"{"name": "test", "value": 42}"#;
    
    let mut cmd = Command::cargo_bin("rcli").unwrap();
    cmd.env("RCLI_STORAGE__DATA_DIR", data_dir.to_str().unwrap())
        .args(&["store", "test_key", test_data]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("rcli").unwrap();
    cmd.env("RCLI_STORAGE__DATA_DIR", data_dir.to_str().unwrap())
        .args(&["get", "test_key"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("42"));
}

#[test]
fn test_list_empty() {
    let temp_dir = TempDir::new().unwrap();
    let data_dir = temp_dir.path().join("data");
    
    let mut cmd = Command::cargo_bin("rcli").unwrap();
    cmd.env("RCLI_STORAGE__DATA_DIR", data_dir.to_str().unwrap())
        .args(&["list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No stored items found"));
}

#[test]
fn test_delete_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let data_dir = temp_dir.path().join("data");
    
    let mut cmd = Command::cargo_bin("rcli").unwrap();
    cmd.env("RCLI_STORAGE__DATA_DIR", data_dir.to_str().unwrap())
        .args(&["delete", "nonexistent_key"]);
    cmd.assert().failure();
}