fn main(){
    let mut s1 = String::from("hello");
    change_str(&mut s1);
}

fn change_str(s:&mut String){
    s.push_str("妹子");
    //cannot borrow `*s` as mutable, as it is behind a `&` reference
}

