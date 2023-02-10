fn main() {
    let mut x = String::from("hello world");
    x.insert(5, ',');
    x.insert_str(x.len(), " ajn404");
    eprintln!("{}", x);
    x = x.replace("hello", "wow");
    //该方法是返回一个新的字符串，而不是操作原来的字符串
    dbg!(x);

    let z = "hello hello ajn404 jan404";
    let y = &z.replacen("hello", "hallo", 2);
    dbg!(y);

    let mut w = z.to_string();
    w.replace_range(0..2, "ww");
    dbg!(&w);

    //与字符串删除相关的方法有 4 个，他们分别是 pop()，remove()，truncate()，clear()。这四个方法仅适用于 String 类型。

    eprintln!("关于删除");
    let pop_w = w.pop();
    let pop_w_again = w.pop();
    dbg!(w);
    dbg!(pop_w);
    dbg!(pop_w_again);

    //remove
    //remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    //例如

    {
        let mut to_be_removed_str = String::from("是艰苦的跋涉jajaj");
        eprintln!(
            "to_be_removed_str 占用的字节数:{}",
            std::mem::size_of_val(&to_be_removed_str)
        );
        to_be_removed_str.remove(0);
        eprintln!(
            "to_be_removed_str 占用的字节数:{}",
            std::mem::size_of_val(&to_be_removed_str)
        );
        dbg!(&to_be_removed_str);
        to_be_removed_str.remove(0);
        eprintln!(
            "to_be_removed_str 占用的字节数:{}",
            std::mem::size_of_val(&to_be_removed_str)
        );
        dbg!(&to_be_removed_str);
        eprintln!(
            "to_be_removed_str 占用的字节数:{}",
            std::mem::size_of_val(&to_be_removed_str)
        );
    }

    //truncate: 截断
    //删除字符串中从指定位置开始到结尾的全部字符
    //该方法 truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。

    {
        let mut syr_truncate = "23我爱1232".to_string();
        syr_truncate.truncate(5);
        eprintln!("about truncate");
        dbg!(syr_truncate);
    }

    //clear
    {
        let mut to_be_clear = "函数就大手大脚".to_string();
        eprintln!("about clear");
        to_be_clear.clear();
        dbg!(to_be_clear);
    }

    //format!
    {
        let h1 = "123".to_string();
        let h2 = "456".to_string();
        let h = format!("{}-{}", h1, h2);
        dbg!(h);
    }

    //字符串转义
    {
        println!("{}", "hello \\x52\\x75\\x73\\x74");
        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("{}", raw_str);

        // 如果字符串包含双引号，可以在开头和结尾加 #
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        // 如果还是有歧义，可以继续增加，没有限制
        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", longer_delimiter);
    }
}
