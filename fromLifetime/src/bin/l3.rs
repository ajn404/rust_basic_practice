//结构体的生命周期标注


fn main(){
    let novel = String::from("call me Ishmael.Some years ago...");
    let first_sentence = novel.split('.').next().expect("counld not find a '.'");


    let i = ImportantExcerpt{
        part:first_sentence
    }; 


    // first_sentence的生命周期覆盖了i的生命周期，所以是没有问题的



}



struct ImportantExcerpt<'a>{
    part:&'a str,
}


