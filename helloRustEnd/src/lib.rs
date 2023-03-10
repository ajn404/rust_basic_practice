#[cfg(test)]
mod vector {
    #[test]
    fn create_vec() {
        let v: Vec<i32> = Vec::new();
        let mut x = Vec::new();
        x.push(1);

        let z: Vec<String> = Vec::with_capacity(10);
        dbg!(z);

        let mut t = vec![1, 2, 3, 4];
        t.push(100);
        dbg!(&t);
        let third = &t[2];
        eprintln!("t的第三个元素是{}", third);
        if let Some(third) = t.get(2) {
            eprintln!("第三个元素是{third}");
        }

        //get方法，有值时返回Some(index),无值时返回None,不会发生数组越界的报错
    }

    #[test]
    fn attention_please() {
        let mut v = vec![1, 2, 3, 4];
        let first = &v[0];
        dbg!(first);
        v.push(5);
        // dbg!(first);
        //数组的大小是可变的，当旧数组的大小不够用时，
        //Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。
    }

    #[test]
    fn iter_vec() {
        let v = vec![1, 2, 3, 4];
        for item in &v {
            eprintln!("{item}");
        }

        let mut vv = vec![1, 2, 3];
        for item in &mut vv {
            eprintln!("{item}");
            *item += 10;
        }
    }

    #[test]
    fn storage_different_type_use_enum() {
        #[derive(Debug)]
        enum IPAdrr {
            V4(String),
            V6(String),
        }

        let v = vec![
            IPAdrr::V4("127.1.1.1".to_string()),
            IPAdrr::V6("::1".to_string()),
        ];
        for p in v {
            show_addr(p);
        }

        fn show_addr(ip: IPAdrr) {
            eprintln!("{:?}", ip);
        }
    }

    #[test]

    fn storage_different_type_use_trait() {
        trait IpAddr {
            fn display(&self);
        }

        struct V4(String);
        impl IpAddr for V4 {
            fn display(&self) {
                eprintln!("ipv4{:?}", self.0);
            }
        }

        struct V6(String);
        impl IpAddr for V6 {
            fn display(&self) {
                eprintln!("ipv6 {:?}", self.0);
            }
        }

        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::2".to_string())),
        ];

        for ip in v {
            ip.display();
        }
    }
}

#[cfg(test)]
mod hashmap {
    use std::collections::HashMap;
    #[test]

    fn create_hashmap() {
        let mut my_gems = HashMap::new();
        my_gems.insert("红宝石", 1);
        my_gems.insert("蓝宝石", 2);
        my_gems.insert("河边。。。破石头", 18);
        //HashMap 也是内聚性的，即所有的 K 必须拥有同样的类型，V 也是如此。
    }

    #[test]
    fn hashmap_example() {
        let team_list = vec![
            ("中国队".to_string(), 100),
            ("美国队".to_string(), 1),
            ("日本队".to_string(), 2),
        ];
        let teams_map: HashMap<_, _> = team_list.into_iter().collect();
        /*
        collect 方法在内部实际上支持生成多种类型的目标集合，因此我们需要通过类型标注
            HashMap<_,_> 来告诉编译器：请帮我们收集为
        HashMap 集合类型，具体的 KV 类型，麻烦编译器您老人家帮我们推导。
         */
        eprintln!("{:?}", teams_map);
    }

    #[test]
    fn owner_tranlate() {
        let name = "me".to_string();
        let age = 1000;
        let mut handsome_boy = HashMap::new();
        handsome_boy.insert(name, age);
        dbg!(handsome_boy);
        // eprintln!("{name}");
        dbg!(age);
        //borrow of moved value: `name`

        //使用引用类型放入HashMap中,请确保gai
    }

    #[test]
    fn getHashmap() {
        let mut scotes = HashMap::new();
        scotes.insert("dasdasdsa".to_string(), 10);
        scotes.insert("dasdasd11sa".to_string(), 10);
        dbg!(&scotes);
        let keyName = String::from("23213sd");
        let scote = &scotes.get(&keyName);
        dbg!(scote);

        for (key, value) in &scotes {
            eprintln!("{}:{}", key, value);
        }

        let old = scotes.insert("abc".to_string(), 2123);
        dbg!(old);
        //None
        let old = scotes.insert("abc".to_string(), 2123);
        dbg!(old);
        //Some(2123)
        let v = scotes.entry("bca".to_string()).or_insert(5);
        dbg!(v);
    }

    #[test]

    fn statistics_words() {
        let words = "hello chatgpt hello world";
        let mut map: HashMap<String, i32> = HashMap::new();
        for word in words.split_whitespace().into_iter() {
            let count = map.entry(word.to_string()).or_insert(0);
            //前插入过则取出之前统计的该词语出现的次数，对其加一。
            //or_insert 返回了 &mut v 引用，因此可以通过该可变引用直接修改 map 中对应的值
            *count+=1;
        }

        dbg!(map);
    }
}
