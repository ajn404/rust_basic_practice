use std::{fs::File, io::ErrorKind};

fn main() {
    // let f = File::open("hh.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         match err.kind() {
    //             ErrorKind::NotFound =>{
    //                 match File::create("hh.text") {
    //                     Ok(fc)=>fc,
    //                      Err(e)=>panic!("err create file {:?}",e)
    //                 }
    //             }
    //             other_err =>{
    //                 panic!("error open file{:?}",other_err);
    //             }
    //         }
    //     }
    // };

    let path = "simpleHH.txt";

    let f = File::open(&path).unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create(&path).unwrap_or_else(|error|{
                panic!("{:?}",error);
            })

        } 
        else{
            panic!("error {:?}",error);
        }
    });
}
/*
thread 'main' panicked at 'Error open file Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/bin/Result_enum.rs:8:13
*/
