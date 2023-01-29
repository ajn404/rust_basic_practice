
fn main(){
    let  s = String::from("hello world");
    let first_index = first_index_fun(&s[..]);

    let x = "你说的 对！";
    let first_index_x = first_index_fun(x);
    // s.clear();
    //cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("\n结果{}\n{}",first_index,first_index_x);
}

fn first_index_fun(s:&str)->&str{
    let byte = s.as_bytes();
    for (i,&item) in byte.iter().enumerate(){
        print!("\n遍历i:{}\t item:{}\t &item:{} \t item == b' ':{}",i,item,&item,item==b' ');
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

//定义函数时使用字符串切片代替字符串引用会使api更通用，且不会损失任何功能 
