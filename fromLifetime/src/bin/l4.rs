//生命周期省略规则

fn first_index(s: &String) -> &str {
    let byte = s.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {} 


//  https://www.cnblogs.com/lilpig/p/17014977.html