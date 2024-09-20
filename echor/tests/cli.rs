use predicates::prelude::predicate;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn fail_with_no_arguments() -> Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn success_with_argument() -> Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("echor")?;
    cmd.arg("Hello, world!").assert().success();
    Ok(())
}

#[test]
fn generate_correct_output_1() -> Result<()> {
    let expected = std::fs::read_to_string("tests/expected/hello1.txt")?;
    let mut cmd = assert_cmd::Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn generate_correct_output_2() -> Result<()> {
    let expected = std::fs::read_to_string("tests/expected/hello2.txt")?;
    let mut cmd = assert_cmd::Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn generate_correct_output_1_no_newline() -> Result<()> {
    let expected = std::fs::read_to_string("tests/expected/hello1.n.txt")?;
    let mut cmd = assert_cmd::Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello there", "-n"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn generate_correct_output_2_no_newline() -> Result<()> {
    let expected = std::fs::read_to_string("tests/expected/hello2.n.txt")?;
    let mut cmd = assert_cmd::Command::cargo_bin("echor")?;
    cmd.args(vec!["-n", "Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
