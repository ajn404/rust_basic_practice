fn main(){
    let s1 = String::from("hello");
    changeStr(&s1)
}

fn changeStr(s:&String){
    s.push_str("妹子");
    //cannot borrow `*s` as mutable, as it is behind a `&` reference
}


//只需要一个小小的调整，即可改变上述错误，见a4