//因为切片是对集合的部分引用，因此不仅仅字符串有切片，其它集合类型也有，例如数组：
fn main(){
    let a = [1,2,3,4,5,6];
    let slice = &a[1..3];
    assert_eq!(slice,&[2,3]);
}