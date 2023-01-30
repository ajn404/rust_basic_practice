//match匹配必须穷举所有可能性
// 下划线通配符
fn main(){
    let v = 0u8;
    match v {
        1=>println!("one"),
        _=>println!("not one maye zero maybe other"),
    }
}