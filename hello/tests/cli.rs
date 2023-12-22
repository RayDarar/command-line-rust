use assert_cmd::Command;

#[test]
fn it_works() {
    let mut cmd = Command::cargo_bin("hello").unwrap();

    cmd.assert().success();

    let output = cmd.output().unwrap();
    
    assert_eq!(output.stdout, b"Hello, world!\n")
}
