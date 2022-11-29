// 使用两种方法让代码工作起来
fn main() {
    let v:i32= {
        let mut x = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
    println!("{}",v)
 }
 