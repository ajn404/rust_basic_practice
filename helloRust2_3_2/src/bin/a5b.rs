#![allow(unused)]
fn main() {
    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    let s3 = &mut s;
    // cannot borrow `s` as mutable because it is also borrowed as immutable
    // 无法借用可变 `s` 因为它已经被借用了不可变
    println!("s1:{}\ns2:{}\ns3:{}",s1,s2,s3);
}

