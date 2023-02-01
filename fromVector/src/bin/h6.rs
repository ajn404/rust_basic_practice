use std::collections::HashMap;
//替换现有的v
fn main() {
    let text = "hello world  cancel shit";

    let mut hs = HashMap::new();
    for word in text.split_whitespace(){
        let count  = hs.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}",hs);


}
