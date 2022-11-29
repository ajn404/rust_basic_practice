fn main(){
    forever();
    trueforever();
}

//单元类型：无返回值
fn forever()->(){
    loop{
        print!("1");
        break;
    }
}

//还有一种永不返回的发散函数
fn trueforever()->!{
    loop{
        print!("1");
    }
}
