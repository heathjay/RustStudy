fn main() {
    //panic
    let v = vec![1,2,3];
    //println!("{}",v[99]);
    //backtrack
    use std::fs::File;
    //Result
/*    
    let f = File::open("hello.rs");
    //match:result
    let f = match f{
        Ok(file) => file,
        Err(error) => {panic!("err:{:?}",error);},
    };
*/
/*
     //deal with different result
    let f = File::open("hello.rs");
    use std::io::ErrorKind;
    //match:result
    let f = match f{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.rs"){
                Ok(file) => file,
                Err(e) => panic!("panic = {}",e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        }
    };
*/
    //unwrap result
    //unwrap->ok -> message
    //unwrap->err -> panic!
    let f = File::open("./good.rs").unwrap();
    //expect("err info")
    let f = File::open("hello.rs").expect("no such file");
    
}
