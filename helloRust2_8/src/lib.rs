

#[cfg(test)]
mod testtttt{
    #[test]
    fn about_generics(){


        fn largest<T:std::cmp::PartialOrd+Copy>(list:&[T])->T
        // where
        // T:std::cmp::PartialOrd
        {
            let mut res = list[0];
            for &item in list.iter(){
                if item > res{
                    res = item
                }
            }
            res
        }


        let number_list = vec![1,2,3,4];
        let res = largest(&number_list);
        dbg!(res);

    }

    #[test]

    fn struct_generics(){
        #[derive(Debug)]
        struct Point<T>{
            x:T,
            y:T,
        }

        let int = Point{x:1,y:2};
        let flo = Point{x:1.1,y:1.2};
        dbg!((&int,&flo));
    }


    #[test] 
    fn struct_generics_different(){
        #[derive(Debug)]
        struct Point<T,U>{
            x:T,
            y:U,
        }

        let x = Point{
            x:1,
            y:2.2,
        };
        dbg!(x);
    }


    #[test]
    fn dragon_enum_phoenix_generics(){
        enum Option<T>{
            Some(T),
            None
        }

        enum Result<T,E>{
            Ok(T),
            Err(E),
        }

    }

    #[test]
    fn impl_generics(){
        #[derive(Debug)]
        struct Point<T>{
            x:T,
            y:T,
        }

        impl <T> Point<T>{
            fn x(&self) -> &T{
                &self.x
            }
        }

        let p  = Point{
            x:20,
            y:20,
        };
        eprintln!("p.x() = {}",p.x());
    }







}