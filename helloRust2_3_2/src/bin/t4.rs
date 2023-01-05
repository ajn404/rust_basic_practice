
// 修复错误
fn main() {
    let mut s = String::from("hello, ");
    println!("{}",push_str(&mut s))
}

fn push_str(s: &mut String)->&mut String {
    s.push_str("world");
    s
}
