use std::collections::HashMap;
//遍历
fn main() {
    let mut hs = HashMap::new();
    hs.insert(String::from("heyheyhye"), 100);
    hs.insert(String::from("weiweiwie"), 1000);
    // down here hs has moved
    for (k,v) in hs {
        println!("{}:{}",k,v);
    }
}
