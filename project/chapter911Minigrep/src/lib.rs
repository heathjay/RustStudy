use std::error::Error;
use std::fs;
use std::env;
pub struct Config{
    pub query : String,
    pub filename: String,
    pub case_sensitive: bool,
}
/*
impl Config{
    pub fn new(args:&[String]) -> Result<Config, &'static str>{
        //错误信息进行修复
        //panic
        if args.len() < 3{
           return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query,filename,case_sensitive})
    }
}
*/

//进行改进
//有所有权的迭代器作为参数而不是借用slice。
//我们将使用迭代器功能之前检查slice长度和索引特定位置的代码，这会明确Config::new的工作因为迭代器会负责访问这些值
impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>{
        args.next();
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("didnot get a query string."),
        };

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("didnot get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query, filename, case_sensitive})
    }
}
pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
    //省略错信息
    let contents = fs::read_to_string("poem.txt")?;
    
    let results = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for line in results{
        println!("{}",line);
    }
    Ok(())
}
/*
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
*/

pub fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query)).collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."],
            search(query,contents)
        );
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";        
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}