use std::collections::HashMap;

fn main(){
    let teams = vec![String::from("hahah"),String::from("wawaa")];
    let intial_scores = vec![10,50];
    let scores:HashMap<_,_> = teams.iter().zip(intial_scores.iter()).collect();

    //<_,_>推导类型
    

}