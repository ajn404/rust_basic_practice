use std::collections::HashMap;
//获取HashMap
fn main (){
    let mut hs = HashMap::new();
    hs.insert(String::from("heyheyhye"), 100);
    hs.insert(String::from("weiweiwie"), 1000);
    //hashMap用的比较少，不在预导入模块

    let team_name = String::from("heyheyhye");
    let heyheyhye = hs.get(&team_name);
    //get返回的是Option
    match heyheyhye {
        Some(s)=>println!("i get it {}",s),
        None=>println!("team not exist"),
    }
}