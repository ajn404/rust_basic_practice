//闭包
//捕获环境
fn main() {
    let x = 1;
    // fn equal_to_x(z: u32) -> bool {
    //     x == z

    //     /*
    //             can't capture dynamic environment in a fn item
    //     use the `|| { ... }` closure form instead
    //             */


    // }

    let z = |z| z==x;
    println!("{}",z(1));
    //true
}


 