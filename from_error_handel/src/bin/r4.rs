use std::error::Error;
use std::fs::File;

fn main()-> Result<(),Box<dyn Error>>{
    let mut f = File::open("hh.txt")?;
    Ok(())
}
