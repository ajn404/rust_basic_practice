#[cfg(test)]
mod tests {

    #[test]
    fn about_match() {
        enum Action {
            Say(String),
            MoveTo(i32, i32),
            ChangeColorRGB(u8, u8, u8),
        }

        let actions = [
            Action::Say("hello".to_string()),
            Action::MoveTo(12, 12),
            Action::ChangeColorRGB(255, 255, 255),
        ];

        for action in actions {
            match action {
                Action::Say(s) => {
                    dbg!(s);
                }
                Action::MoveTo(x, y) => {
                    dbg!(x);
                    dbg!(y);
                }
                Action::ChangeColorRGB(r, g, b) => {
                    dbg!([r, g, b]);
                }
            }
        }
    }

    #[test]
    fn about_if_let() {
        let x = Some(3u8);
        if let Some(3) = x {
            dbg!(x);
        }

        if let Some(y) = x {
            dbg!(y);
        }
    }

    #[test]
    fn about_matches_micro() {
        #[derive(Debug)]
        enum MyEnum {
            Foo,
            Bar,
        }

        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Bar];
        for (i, item) in v.iter().filter(|x| matches!(x, MyEnum::Bar)).enumerate() {
            dbg!(i);
            dbg!(item);
        }

        let foo = 'f';
        dbg!(matches!(foo, 'A'..='Z'));
        // matches!(foo, 'A' ..= 'Z') = false
        dbg!(matches!(foo,'A'..='Z'|'a'..='z'));
        // matches!(foo, 'A' ..= 'Z' | 'a' ..= 'z') = true

        let bar = Some(4);
        dbg!(matches!(bar,Some(x) if x>2));
        //true
    }

    //2.6.2解构Option

    #[test]
    fn about_deconstruction() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                Some(i) => Some(i + 1),
                None => None,
            }
        }

        let x = Some(100);
        let x_plus = plus_one(x);
        let nn = plus_one(None);
        dbg!([x, x_plus, nn]);
    }

    #[test]
    fn about_while_let() {
        let mut stack = vec![1, 2, 3];
        while let Some(top) = stack.pop() {
            dbg!(top);
        }
    }

    //2.6.4全模式列表
    #[test]
    fn all_pattern() {
        eprintln!("字面量匹配");
        //字面量匹配
        {
            let x = 1;
            match x {
                1 => {
                    dbg!(x);
                }
                _ => {
                    eprintln!("what'd the hell");
                }
            }
        }
        //匹配命名变量
        eprintln!("匹配命名变量");

        {
            let a = Some(5);
            let b = 10;
            match a {
                Some(50) => {
                    dbg!(a);
                }
                Some(b) => {
                    eprintln!("b:{}", b);
                }
                _ => {
                    dbg!(a);
                }
            }

            dbg!(a);
            dbg!(b);
        }
        eprintln!("单分支多模式");
        //单分支多模式
        {
            let m = 1;
            match m {
                1 | 2 | 3 => {
                    dbg!(m);
                }
                _ => {
                    eprintln!("_");
                }
            }
        }

        eprintln!("通过序列..=匹配值的范围");
        {
            let x = 5;
            match x {
                1..=4 => eprintln!("1..4"),
                _ => eprintln!("_"),
            }

            let s = 'Z';
            match s {
                'a'..='z' => eprintln!("'a'..='z'"),
                'A'..='Z' => eprintln!("'A'..='Z'"),
                _ => eprintln!("_"),
            }
        }
    }

    #[test]
    fn struct_destruction() {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 100, y: 199 };
        {
            let Point { x: a, y: b } = p;
            dbg!([a, b]);
            let Point { x, y } = p;
            dbg!([x, y]);
        }

        {
            match p {
                Point { x: 100, y } => eprintln!("Point x:100, y:{}", y),
                _ => eprintln!("_"),
            }
        }

        {
            if let Point { x: 100, y } = p {
                eprintln!("if let hahiah y:{}", y);
            }
        }
    }

    #[test]
    fn nested_struct_enum_destruction() {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        {
            let msg = Message::ChangeColor(Color::Hsv(10, 160, 255));
            match msg {
                Message::ChangeColor(Color::Rgb(r, g, b)) => {
                    dbg!([r, g, b]);
                }
                _ => {
                    eprintln!("others");
                }
            }
        }
    }

    #[test]
    fn enum_with_struct_complicated_destruction() {
        struct Point {
            x: i32,
            y: i32,
        }
        let ((feet, inches), Point { x, y }) = ((30, 20), Point { x: 3, y: 20 });
        dbg!([feet, inches, x, y]);
    }

    #[test]
    fn array_destruction() {
        let arr = [114, 421];
        let [x, y] = arr;
        dbg!([x, y]);
    }

    #[test]
    fn var_length_array_destruction() {
        let arr = &[114, 115];
        if let [x, ..] = arr {
            dbg!(x);
            assert_eq!(x, &114);
        }

        if let &[.., y] = arr {
            dbg!(y);
        }
        let arr: &[u16] = &[];
        assert!(matches!(arr, [..]));
        assert!(!matches!(arr, [x, ..]));
    }

    #[test]
    fn _ignore() {
        fn foo(_: i32, y: i32) {
            println!("{}", y);
        }
        //比如实现特征时，当你需要特定类型签名但是函数实现并不需要某个参数时。
        //此时编译器就不会警告说存在未使用的函数参数，就跟使用命名参数一样。
        foo(3, 4);
    }

    #[test]
    fn nested_ignored() {
        let mut setting_value = Some(5);
        let new_setting_value = Some(100);
        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                eprintln!("can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        dbg!(setting_value);

        //模式匹配一定要类型相同，因此匹配 numbers
        //元组的模式，也必须有五个值（元组中元素的数量也属于元组类型的一部分）
    }

    #[test]
    fn lazy_name() {
        let s = Some("hhhhh".to_string());
        if let Some(_s) = s {
            eprintln!("hahhia");
        }

        //borrow of partially moved value: `s`
        // eprintln!("{:?}",s);
        let x = Some("hhhhh".to_string());
        if let Some(_) = x {
            eprintln!("hahhia");
        }

        eprintln!("{:?}", x);
    }

    #[test]
    fn lazy_name_about_dot_dot() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point { x: 0, y: 20, z: 11 };
        match origin {
            Point { x, .. } => {
                dbg!(origin);
            }
        }

        let numbers = (1, 2, 3, 4, 5);
        if let (a, .., b) = numbers {
            dbg!(numbers);
        }
    }

    #[test]
    //匹配守卫提供的额外条件
    fn guard_pattern() {
        let num = Some(4);
        match num {
            Some(n) if n <= 4 => {
                eprintln!("离谱 n={}", n);
            }
            _ => {
                eprintln!("其他")
            }
        }
    }

    #[test]
    fn guard_pattern_complicated() {
        let x = 4;
        let y = true;
        match x {
            1..=6 if y => {
                eprintln!("yes");
            }
            _ => eprintln!("no"),
        }
    }

    #[test]
    fn at_bundle() {
        #[derive(Debug)]
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };
        match msg {
            Message::Hello {
                id: id_variable @ 1..=2,
            } => {
                dbg!(msg);
                dbg!(id_variable);
            },

            Message::Hello { id: 2..=5 } => {
                eprintln!("second match");
            },
            Message::Hello { id }=>{
                dbg!(id);
            },

            _=>{
                eprintln!("hhh");
            },
        }
    }
}
