fn main(){
    let a = Box::new(3);
    println!("{}",a);
    //println! 可以正常打印出 a 的值，是因为它隐式地调用了 Deref 对智能指针 a 进行了解引用

}