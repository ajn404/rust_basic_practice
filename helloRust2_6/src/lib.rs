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
    fn enum_with_struct_complicated_destruction(){
        struct Point{
            x:i32,y:i32,
        }
        let ((feet,inches),Point { x, y }) = ((30,20),Point{x:3,y:20});
        dbg!([feet,inches,x,y]);
    }

    #[test]
    fn array_destruction(){
        let arr = [114,421];
        let [x,y] = arr;
        dbg!([x,y]);
    }

    #[test]
    fn var_length_array_destruction(){
        let arr = &[114,115];
        if let [x,..] = arr{
            dbg!(x);
            assert_eq!(x,&114);
        }

        if let &[..,y] = arr{
            dbg!(y);
        }


        let arr:&[u16] = &[];
        assert!(matches!(arr,[..]));
        assert!(!matches!(arr, [x,..]));

    }



}
