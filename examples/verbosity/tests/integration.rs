extern crate assert_cli;

#[test]
fn no_v() {
    assert_cli::Assert::main_binary()
        .stderr().contains("always shown")
        .stdout().is("0 verbose flags passed
error logging enabled
warn debugging disabled
info debugging disabled
debug debugging disabled
trace debugging disabled")
        .unwrap();
}

#[test]
fn v() {
    assert_cli::Assert::main_binary()
        .with_args(&["-v"])
        .stderr().contains("always shown")
        .stderr().contains("shown at -v")
        .stderr().doesnt_contain("shown at -vv")
        .unwrap();
}

#[test]
fn vv() {
    assert_cli::Assert::main_binary()
        .with_args(&["-vv"])
        .stderr().contains("always shown")
        .stderr().contains("shown at -vv")
        .stderr().doesnt_contain("shown at -vvv")
        .unwrap();
}

#[test]
fn vvv() {
    assert_cli::Assert::main_binary()
        .with_args(&["-vvv"])
        .stderr().contains("always shown")
        .stderr().contains("shown at -vvv")
        .stderr().doesnt_contain("shown at -vvvv")
        .unwrap();
}

#[test]
fn vvvv() {
    assert_cli::Assert::main_binary()
        .with_args(&["-vvvv"])
        .stderr().contains("always shown")
        .stderr().contains("shown at -vvvv")
        .stdout().doesnt_contain("trace debugging disabled")
        .unwrap();
}
