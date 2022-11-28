fn main(){
    let a = 'a';
    let b = '啵';
    let c = '🐱';
    println!("{},{},{}",a,b,c);
}
//所有的 Unicode 值都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型