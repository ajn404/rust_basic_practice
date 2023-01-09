fn main(){
    let x = String::from("hello world");
    let slice_x = &x[1..2];
    let slice_copy_x =&x[..];
    let y = &x[..2];
    let z = &x[0..10];
    let w = &x[0..];
    assert_eq!(x,w);
    println!("{}\n{}\n {}\n{}",slice_x,slice_copy_x,y,z);
}