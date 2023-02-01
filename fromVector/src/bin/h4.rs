use std::collections::HashMap;
//替换现有的v
fn main() {
    let mut hs = HashMap::new();
    hs.insert(String::from("heyheyhye"), 100);
    hs.insert(String::from("heyheyhye"), 1000);
    println!("{:?}",hs);
    // /{"heyheyhye": 1000}
}
