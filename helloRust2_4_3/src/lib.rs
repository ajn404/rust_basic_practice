struct User {
    name: String,
    mobile: String,
    boy: bool,
    age: usize,
    des: String,
}

#[derive(Debug)]

struct File {
    name: String,
    data: Vec<u8>,
}

#[cfg(test)]
mod test_struct {

    use super::*;
    #[test]

    fn haihai() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn create_strct() {
        let mut user1 = User {
            name: "ajn404".to_string(),
            mobile: String::from("1551888877777"),
            boy: false,
            age: 99,
            des: String::from("a pure man"),
        };

        fn change_user_name(originUser: &mut User, name: String) -> &User {
            let tempUser = originUser;
            tempUser.name = name;
            tempUser
        }

        let new_user = change_user_name(&mut user1, String::from("jiu"));
        dbg!(&new_user.name);
    }

    #[test]
    fn about_struvt_01() {
        let mut user1 = User {
            name: "ajn404".to_string(),
            mobile: String::from("1551888877777"),
            boy: false,
            age: 99,
            des: String::from("a pure man"),
        };

        let user2 = User {
            name: "666999333".to_string(),
            ..user1
        };
    }

    #[test]
    fn about_struvt_02() {
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };
        eprintln!("{}", user1.active);
        // 下面这行会报错
        // eprintln!("{}", user1.username);
        dbg!(user1.sign_in_count);
        // 下面这行会报错
        // println!("{:?}", user1);
    }

    #[test]
    fn about_struvt_03() {
        let f1 = File {
            name: String::from("f1.txt"),
            data: Vec::new(),
        };

        let f1_length = &f1.data.len();
        let f1_name = &f1.name;
        println!("f1 is {:?}", f1);
        println!("{} is {} bytes long", f1_name, f1_length);
    }

    #[test]
    fn tupleStruct() {
        struct Color(i32, i32, i32);
        let c = Color(12, 22, 33);
        dbg!(c.0);
    }

    #[test]
    fn unit_like_struct() {
        #[derive(Debug)]
        pub struct AlwaysEqual;
        pub trait haiha {
            //这是因为在Rust中，特质(trait)方法必须被定义为实例方法(associated method)，也就是说它们必须有一个参数，该参数表示方法所属的类型。这个参数称为Self，它可以是具体类型、引用、借用等形式。
            fn mma(&self) {}
        }
        impl haiha for AlwaysEqual {
            fn mma(&self) {
                dbg!("hahah");
            }
        }
        let a = AlwaysEqual;
        println!("www{:?}", a);
        a.mma();
    }


    #[test]
    fn about_struct_04(){
        #[derive(Debug)]
        // /该生命周期标注说明，结构体 所引用的字符串 str 必须比该结构体活得更久
        struct User<'a>{
            username:&'a str,
            email:&'a str,
            sign_in_count:u64,
            active:bool,
        }

        let user = User{
            username:"hh@hh.com",
            email:"wwww",
            sign_in_count:32,
            active:false,
        };
        dbg!(&user);
        println!("{:?}",&user);
        //更好的输出表现
        println!("{:#?}",&user);
        
    }


    #[test]
    fn useDebug(){
        #[derive(Debug)]
        struct Rectangle{
            with:u32,
            height:u32,
        }

        let scale = 2;
        let rect1 = Rectangle{
            with:100,
            height:dbg!(scale*30),
        };
        dbg!(&rect1);
    }
}
