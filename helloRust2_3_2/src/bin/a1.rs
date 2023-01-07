fn main(){
    let x = 5;
    let y = &x;
    assert_eq!(x,5);
    assert_eq!(*y,5);
}