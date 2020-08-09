use std::env;
//use std::fs;
use std::process;
//use std::error::Error;

use chapter911Minigrep::Config;
fn main() {

    
   //let config = parse_config(&args);
   /*panic
   let config = Config::new(&args);
    println!("searching for {}", config.query);
    println!("in file {}",config.filename);

    let contents = fs::read_to_string(config.filename).expect("something went wrong reading the file!");
    println!("with test:{}", contents);
    */

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //println!("searching for {}", config.query);
    //println!("in file {}",config.filename);
    /*
        把业务逻辑提取
    let contents = fs::read_to_string(config.filename).expect("something went wrong reading the file!");
    println!("with test:{}", contents);

    */
    
    //用if let去检查是否返回一个err
    if let Err(e) = chapter911Minigrep::run(config){
        //println!("app err:{}",e);
        eprintln!("app err:{}",e);
        process::exit(1);
    }
}




/*

提取到lib里面
//3.1 新建结构体来表示
struct Config{
    query : String,
    filename :String,
}
//3.函数剥离
// fn parse_config(args: &[String]) -> (&str, &str){
//     let query = &args[1];
//     let filename = &args[2];
//     (query,filename)
// }
*/
//3.2 结构体new函数
/*
impl Config{
    fn new(args:&[String]) -> Config{
        //错误信息进行修复
        //panic
        if args.len() < 3{
            panic!("not enough args!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config{query,filename}
    }
}
*/
/*
提取到lib
impl Config{
    fn new(args:&[String]) -> Result<Config, &'static str>{
        //错误信息进行修复
        //panic
        if args.len() < 3{
           return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query,filename})
    }
}
*/
// fn parse_config(args: &[String]) -> Config{
//     //只允许借用的情况下，进行clone，新建一个
//     // let query = args[1].clone();
//     // let filename = args[2].clone();
//     Config::new(args)
// }


/*
提取到lib
//4. 进行run函数的提取
    //实现trait对象 Box<dyn Error>
    //返回实现了error trait的类型
    //dynamic
    //？返回错误值并让调用者来处理他

fn run(config:Config) -> Result<(),Box<dyn Error>>{
    //省略错信息
    let contents = fs::read_to_string("poem.txt")?;
    println!("with test:{}", contents);
    Ok(())
}
*/