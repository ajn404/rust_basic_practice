
// 修改 `assert!` 让代码工作
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{}",0xff);//255
    //63
    // 255
    assert!(v == 1597);
}
