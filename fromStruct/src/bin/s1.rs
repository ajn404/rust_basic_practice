struct User{
    name:String,
    email:String,
    age:u64,active:bool,
}


fn build_user(name:String,email:String)->User{
    User { name, email, age: 10, active: false }
}




fn main(){
    let user1 = build_user(String::from("jane"), String::from("jane@ajn404.com"));
    print!("{:?}",user1.name);
} 