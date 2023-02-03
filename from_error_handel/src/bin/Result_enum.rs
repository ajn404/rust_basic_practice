use std::fs::File;

fn main() {
    let f = File::open("hh.txt");
    let _f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("Error open file {:?}", err);
        }
    };
}

/*
thread 'main' panicked at 'Error open file Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/bin/Result_enum.rs:8:13
*/
