

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

    

}
