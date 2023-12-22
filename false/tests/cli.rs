use assert_cmd::Command;

#[test]
fn it_works() {
    let code = Command::cargo_bin("true")
        .unwrap()
        .output()
        .unwrap()
        .status
        .code()
        .unwrap();

    assert!(code > 0);
}
