
struct User{
    name:String,
    email:String,
    age:u64,active:bool,
}

fn main(){
    let user1 = User { name:String::from("jane"), email:String::from("jane"), age: 10, active: false };
    let user2 = User { name:String::from("ajn404"),..user1};
    print!("{}{}",user2.name,user2.age);  
    
} 