use std::env;
use minigrep::Config;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else({
        |err| {
            //这个小项目的所有错误信息都集中在本文件中
            //eprint用于将错误输出到标准错误中，即控制台
            //而print用于输出到标准输入中
            //例如 cargo run three target.txt > output.txt
            //nice
            eprintln!("problem parsing arguments : {}", err);
            process::exit(1);
        } 
    });

    // run(config);  

    if let Err(e) = minigrep::run(config)  {
        eprintln!("Application error is : {}",e);
        process::exit(1);
    }
}

