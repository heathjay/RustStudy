use std::env;
use std::fs;
fn main() {
    let args:Vec<String> = env::args().collect();
    
   //let config = parse_config(&args);
   let config = Config::new(&args);
    println!("searching for {}", config.query);
    println!("in file {}",config.filename);

    let contents = fs::read_to_string(config.filename).expect("something went wrong reading the file!");
    println!("with test:{}", contents);
}
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
// fn parse_config(args: &[String]) -> Config{
//     //只允许借用的情况下，进行clone，新建一个
//     // let query = args[1].clone();
//     // let filename = args[2].clone();
//     Config::new(args)
// }