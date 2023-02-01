use std::fs::File;


fn main(){
     let f = File::open("hh.txt");
     match f {
         Ok(file)=>
            file,
         Err(err)=>{
            panic!("Error open file {:?}",err);
         }
     }
}