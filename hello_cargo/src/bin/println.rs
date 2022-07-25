fn main() {
    println!("test println!");
    println!("{}", 1);//默认用法
    println!("{:o}", 8);//8进制
    println!("256的16进制小写为 {:x}", 256);//16进制小写
    println!("{:X}", 256);//16进制大写
    println!("0的指针{:p}", &0);//指针
    println!("18二进制 {:b}", 18);//二进制
    println!("10000科学计数 {:e}", 10000f32);//科学计数
    println!("10000科学计数 (大写){:E}", 10000f32);//科学计数
    println!("{:?}","test"); //打印debug
    println!("{:#?}",("test1","test2"));//带换行和缩进的debug
    println!("{a}{b}",a=19,b="xx");//命名参数
}