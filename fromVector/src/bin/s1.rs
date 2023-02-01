fn main(){
    let s = String::from("hhhh");
    let h = &s[..];
    println!("{}",h);
    // let wh = s[0];

    let len = s.len();
    println!("{}",len); 

    let f = "सुखागत";
    println!("about chars :");
    for item in f.to_string().chars(){
        print!(" \n {}\n",item);
    }
    let splitf = &f[0..3];
    println!("{}",splitf);

    // let splitf = &f[0..2];
    // println!("{}",splitf);

}