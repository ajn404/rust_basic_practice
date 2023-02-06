use testDemo;
mod common;

#[test]

fn use_set_up() {
    common::setup();
    assert_eq!(2 + 2, 4);
}
//运行cargo test才会运行test目录下的文件

// cargo test --test other_intergration_tests
 