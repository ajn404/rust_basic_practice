

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

fn larger<'a>(x: &'a str, y: &str) -> &'a str {
        x
}
//指定生命周期参数的方式依赖于函数所做的事情
 