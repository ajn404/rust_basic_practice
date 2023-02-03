use std::{fs::File, io, io::Read};

// fn read_username_from_file(path: &str) -> Result<String, io::Error> {
//     let f = File::open(path);
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn main() {

//     let res = read_username_from_file("username");  

// }


//上述代码等价于 
fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}

fn main() {

    let res = read_username_from_file("username");  

}