fn main() {
    let x = Box::new(5); //Box存储在堆栈中，它指向的数据存储在堆上

    let mut y = x.clone(); // 完成该行代码，不要修改其它行！

    *y = 4;

    assert_eq!(*x, 5);
    println!("{}", x);
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let add_ptr = add as fn(i32, i32) -> i32;
    // let add_ptr:  fn(i32, i32) -> i32 = add;
    let z = add_ptr(12,21);
    println!("{}", z);

}
