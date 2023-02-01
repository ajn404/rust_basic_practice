//核心语言层面，只有&str或str
//String来自标准语言库

fn main(){
    let x = "dasdasdsa" ;
    println!("{}",x.to_string());   
    let mut y = x.to_string();
    y.push_str(&x.to_string());
    println!("{}",y);
    y.push('a');
    println!("{}",y)    ;


  let s1 = "dsadad".to_string();
  let s2 = "add".to_string();
  let s3 = s1+&s2;
    /*
    deref coercion
    &str 和 &String
    解引用强制转换
    */

//   println!("{}",s1);
  println!("{}",s2);
  println!("{}",s3);


  let h1 = String::from("tic");
  let h2 = String::from("tac");
  let h3 = String::from("toe");

  let h = h1 + "-" + &h2 + "-" +&h3;
  println!("{}",h);

  //h1 被move了

  let fh = format!("{}-{}- {}",h,h2,h3);
  println!("{}",fh);



} 