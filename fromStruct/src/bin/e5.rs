fn main(){
    let v = Some(0u8);

    match v {
        Some(3)=>println!("three"),
        _=>{
            println!("others");
        },
    }

    //与上述match效果相同
    if let Some(3) = v{
        println!("three")
    }else{
        println!("others");
    }
}