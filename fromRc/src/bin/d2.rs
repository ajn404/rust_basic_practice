#[warn(dead_code)]
use std::ops::Deref;

struct MyBox<T>(T);
//元组结构体
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


/*
使用泛型参数前，依然需要提前声明：impl<T>，
只有提前声明了，我们才能在MyBox<T>中使用它，
这样 Rust 就知道 MyBox 的尖括号中的类型是泛型而不是具体类型。
需要注意的是，这里的 MyBox<T> 不再是泛型声明，
而是一个完整的结构体类型，因为我们定义的结构体就是 MyBox<T> 而不再是 MyBox。
*/

impl<T> Deref for MyBox<T> {
    type Target = T;
    //在 Deref 特征中声明了关联类型 Target，关联类型主要是为了提升代码可读性

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {

    let x = MyBox::new(5);
    assert_eq!(5,*x);
}
