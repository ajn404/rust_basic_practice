fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!
               /*
                   cannot borrow `s` as mutable because it is also borrowed as immutable
               mutable borrow occurs here
                   */

    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    &s[..1]
}
