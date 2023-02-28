#[cfg(test)]
mod tests{
    #[test]
    fn for_for_for(){
        for i in 0..=5{
            dbg!(i);
        }
    }

    #[test]
    fn collection_demo(){
        let mut x = [1,2,3,4];
        for item in IntoIterator::into_iter(x){
            dbg!(item);
        }
        //等价
        for item in x{
            dbg!(item);
        }
        for item in &x{
            dbg!(item);
        }
        //等价
        for item in x.iter(){
            dbg!(item);
        }
        for item in &mut x{
            dbg!(item);
        }
        //等价
        for item in x.iter_mut(){
            dbg!(item);
        }
        for (i,v) in x.iter_mut().enumerate(){
            dbg!(v);
            dbg!(i);
        }
    }
    
    #[test]
    fn about_continue(){
        for i  in 1..10{
            if i%2==0{
                continue;
            }
            dbg!(i);
        }
    }

    #[test]
    fn about_break(){
        for i  in 1..10{
            if i%2==0{
                break;
            }
            dbg!(i);
        }
    }

    #[test]

    fn aboutWhile(){
        let mut x = 0;
        while x-100<=0 {
            dbg!(x);
            x+=1;
        }
        eprintln!("over")
    }

    #[test]

    fn about_loop(){
        let mut x =0;
        loop{
            if(x>=100){
                break;
            }
            dbg!(x);
            x+=1;
        }

        eprintln!("over");
    }


    #[test]
    fn loop_exp(){
        let mut x =100;
        let res = loop{
            if(x<=0){
                break x;
            }
            x-=1;
            dbg!(x);
        };
        dbg!(res);
        eprintln!("over");
    }




}