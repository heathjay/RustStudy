fn main() {
    let mut x = 6;
    //let x = 6;
    println!("x = {}",x);
    x = 7;//immutable
    println!("x = {}",x);
    //integer
    let a = 11; //default i32
    println!("a = {}", a);
    let a : i64 = 222;
    println!("a = {}", a);
    let a : i128 = 0xff;
    println!("a = {}", a);
    let a : i64 = 0o721;
    println!("a = {}", a);
  

    //flaot-point
    let f1:f32 = 23.2;
    println!("f1:{}",f1);
    let f1 = 2.2;//default f64
    println!("f1:{}",f1);

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😊';
    println!("heart_eyed_cat:{}",heart_eyed_cat);

    //compound types
    //tuple
    let x1 :(i32, char, f32) = (1, 'A', 28.3);
    let tup = (11,2.3,11);
   // println!("x1 : = {}", x1);
   let (x,y,z) = x1;
   println!("x1.x = {}", x);
   println!("tup.0={}",tup.0);
   //array
   //same data type
   let arr = [1,2,3];
   let a :[i32,5] = [1,2,3,4,5];
}
fn another(x:i32){
    println!("x={}",x);
}