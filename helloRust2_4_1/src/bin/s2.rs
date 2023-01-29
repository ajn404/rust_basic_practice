
fn main(){
    let mut s = String::from("hello world");
    let first_index = first_index(&s);
    // s.clear();
    //cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}",first_index);
}

fn first_index(s:&String)->&str{
    let byte = s.as_bytes();
    for (i,&item) in byte.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// &str字符串切片
// 字符串字面值是不可变的引用
// 形式:【开始索引..结束索引】