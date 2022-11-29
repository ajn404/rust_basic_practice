fn main() {
    #![allow(unused)]
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str() 在字符串后追加字面值
        // s+=", world!";

        println!("{}", s); // 将打印 `hello, world!`
}
