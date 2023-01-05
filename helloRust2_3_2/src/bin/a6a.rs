//悬垂引用
#![allow(unused)]
fn main(){
    let reference_for_nothing  = dangle();
}

fn dangle()-> String{
    // expected named lifetime parameter
    /*
     this function's return type contains a borrowed value, but there is no value for it to be borrowed from help: consider using the `'static` lifetime
    */
    let s = String::from("hello");
    s
}

/*
因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放, 但是此时我们又尝试去返回它的引用。这意味着这个引用会指向一个无效的 String，这可不对！
解决之道看a6a
*/

/*
同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
引用必须总是有效的
*/