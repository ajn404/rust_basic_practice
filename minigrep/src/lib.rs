use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!!");
        }
        // CASE_INSENSITIVE=1 cargo run rust target.txt
        Ok(Config {
            query: args[1].clone(),
            file: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

// Box<dyn Error>::动态地实现了Error trait的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;

    let res = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in res {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for item in contents.lines() {
        if item.contains(query) {
            res.push(item);
        }
    }
    res
    // vec![]
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    let query = query.to_lowercase();

    for item in contents.lines() {
        if item.to_lowercase().contains(&query) {
            res.push(item);
        }
    }
    res
    // vec![]
}

//tdd测试驱动开发
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust;
safe , fast , productive,
pick three
DUCT
ajn404";
        assert_eq!(vec!["safe , fast , productive,"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust;
safe , fast , productive,
pick three
DUCT
ajn404
another line of rusT
";
        assert_eq!(
            vec!["Rust;", "another line of rusT"],
            search_case_insensitive(query, contents)
        );
    }
}

/// Adds one to the number given
///
/// #Examples
///
/// ```
/// let arg = 5
/// let answer = minigrep::add_one(arg);
/// assert_eq!(6,answer);

pub fn add_one(x: i32) -> i32 {
    x + 1
}
