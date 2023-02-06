// fn main() {
//     let str1 = String::from("abcd");
//     {
//         let str2 = "xyz";
//         let res = larger(&str1, str2);
//         println!("the larger one is {}", res);
//     }

fn main() {
    let str1 = String::from("abcd");
    let res ;
    {
        let str2 =String::from("abcd");
        res = larger(str1.as_str(),str2.as_str());

        // let str2 ="abcd";
        // res = larger(&str1, &str2);
    }
    println!("the larger one is {}", res);

}

fn larger<'a>(x: &'a str, y: &'a str) -> &'a str {
    if (x.len() > y.len()) {
        x
    } else {
        y
    }
}

//&i32
//&'a i32
//&'a mut i32

//<'a> 范型生命周期
