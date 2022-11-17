fn main(){
    eprintln!("hh");
    // 忽略未使用的变量，变量名用下划线开头
    let x = 5;
    let _y = 10;
    //变量结构
    let (a,mut b) :(bool,bool) = (true,false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);
    //a = true, b = false
}