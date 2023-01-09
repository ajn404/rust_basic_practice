#![allow(unused)]
fn main() {
    let s = "中国人";
    let a = &s[0..2];
    println!("{}", a);
}

/*

thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside '中' (bytes 0..3) of `中国人`', src/bin/a3a.rs:4:14
stack backtrace:
*/
