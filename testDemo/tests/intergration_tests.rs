use testDemo;

#[test]
fn test() {
    assert_eq!(2 + 2, testDemo::add_2());
}

//运行cargo test才会运行test目录下的文件