fn main(){
    let x = 5;
    let y = &x;
    assert_eq!(x,5);
    assert_eq!(*y,5);
    // assert_eq!(y,5);
    //无法比较整数类型和引用类型
}