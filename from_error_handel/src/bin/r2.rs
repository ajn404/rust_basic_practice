use std::fs::File;

fn main() {
    // let f = File::open("hh.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("Error open file {:?}", err);
    //     }
    // };

    let path = "hh.txt";
        // let f = File::open(path).unwrap();
        let f = File::open(path).expect("无法打开文件");
        //指定panic宏的错误信息
        //thread 'main' panicked at '无法打开文件: Os

}

/*
thread 'main' panicked at 'Error open file Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/bin/Result_enum.rs:8:13
*/
