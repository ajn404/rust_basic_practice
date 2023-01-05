
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    // cannot borrow `s` as mutable more than once at a time
    println!("{}, {}", r1, r2);
}
