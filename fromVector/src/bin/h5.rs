use std::collections::HashMap;
//不替换的时候要使用entry
fn main() {
    let mut hs = HashMap::new();
    hs.insert(String::from("Blue"), 100);


    hs.entry(String::from("Yellow")).or_insert(100);
    hs.entry(String::from("Blue")).or_insert(100);

    let e = hs.entry(String::from("哈哈"));
    //Entry 枚举
    println!("{:?}",e);

    e.or_insert(99);


    

   
    println!("{:?}",hs);
}
