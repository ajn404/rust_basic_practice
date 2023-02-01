fn main() {
    println!("Hello, world!");
    // panic!("crash and burn ");
    //thread 'main' panicked at 'crash and burn ', src/main.rs:3:5

    let v = vec![1,2,3];
    v[5];
    //thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 5', src/main.rs:7:5
    //panic宏出现在依赖的代码
    
    
    // RUST_BACKTRACE设置后可得到回溯信息 
}
 