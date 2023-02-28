use std::string;

#[cfg(test)]

mod tests {
    #[test]
    fn about_array() {
        assert_ne!(1, 2);
        let _x = [99, 22, 33];
        let _y: [i32; 2] = [22, 1];
        let _z = [5; 10];
        for i in _z {
            eprintln!("{}", i);
        }
    }

    #[test]

    fn array_from_fn() {
        let x: [usize; 20] = core::array::from_fn(|i| i);
        func(x);

        let y: [String; 20] = std::array::from_fn(|i| String::from("www"));
        func(y);

        fn func<T>(x: [T; 20])
        where
            T: std::fmt::Display,
        {
            // let x: [usize; 20] = core::array::from_fn(|i| i);
            for i in 0..x.len() {
                eprintln!("{}", x[i]);
            }
        }
    }

    #[test]
    fn comprehensive_example(){
        let one = [1,2,3];
        let two = one;
        let blank1  =[0;3];
        let blank2 = blank1;
        let arrays:[[i32;3];4] = [one,two,blank1,blank2];
        let mut all =0;
        for item in arrays{
            dbg!(item);
            for detail in item{
                dbg!(detail);
                all+=detail;
            }
            let mut sum = 0;
            for index in 0..item.len(){
                sum += item[index];
            }
            dbg!(sum);

        }

        dbg!(all);
    }
}
