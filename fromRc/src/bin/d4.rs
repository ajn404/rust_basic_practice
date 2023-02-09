#[warn(dead_code)]


use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    //在 Deref 特征中声明了关联类型 Target，关联类型主要是为了提升代码可读性

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {
    let s = MyBox::new((String::from("hello world"),String::from("hello world")));
    let h = &*s;
    let (x,y) = h;
    println!("{:?}",&x);


    let w = MyBox::new(String::from("hh"));
    display(&w);

    let w1: &str = &w;
    let w2: String = w.to_string();
    //方法调用会自动解引用

}

fn display (s:&str){
    println!("{}",s)
}


/*
在 Rust 中，
使用 & 表示函数参数是一个引用。
它允许函数使用参数的值，
但是不拥有该值的所有权。
这意味着函数不能修改参数的内容，
因为它不拥有该内容的所有权。
如果函数需要修改参数的内容，
则必须使用 &mut 参数来声明这一点。
*/