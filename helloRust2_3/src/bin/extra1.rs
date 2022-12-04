fn main() {
    let mut c: i32 = 5;
    let mut next: i32 = 0;
    {
        let rc = &mut c;
        next = *rc + 1;
    }
    println!("{}", next);  // 6
}