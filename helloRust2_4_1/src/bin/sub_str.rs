use utf8_slice;

fn main() {
    let s = "holla中国人नमस्ते";
    // for item in s.bytes(){
    //     println!("{}",item);
    // }

    for item in s.chars() {
        println!("{}", item);
    }

    let sub_s = utf8_slice::slice(s, 2, s.len() - 2);
    println!("{}", sub_s);
}
