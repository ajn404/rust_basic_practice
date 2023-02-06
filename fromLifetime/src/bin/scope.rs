fn main() {
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let res = larger(&str1, str2);
    println!("the larger one is {}", res);
}

fn larger<'a>(x: &'a str, y: &'a str) -> &'a str {
    if (x.len() > y.len()) {
        x
    } else {
        y
    }
}
