pub fn sum_2() -> usize {
    1 + 1
}

#[cfg(test)]
mod testssss {
    use super::*;
    #[test]

    fn test_demo() {
        assert_eq!(2, sum_2());
    }



    #[test]
    fn test_tuple_01(){
        let tup = (100,"呵呵",9.9,3/2);
        let (x,y,z,w) = tup;
        //这就是解构：用同样的形式把一个复杂对象中的值匹配出来。
        dbg!(w);
        assert_eq!(x,100);
        assert_eq!(y,"呵呵");
        assert_eq!(z,9.9);
        assert_eq!(w,1);
    }

    #[test]
    fn test_tuple_02(){
        let tup = (500,6.4,1);
        let x = tup.0;
        assert_eq!(x,500);
        // let z = tup.4;
        // dbg!(x);
    }

    




}
