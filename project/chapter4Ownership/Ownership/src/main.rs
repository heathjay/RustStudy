fn main() {

    let mut s = String::from("hello");
    s.push_str(",world!");
    println!("{}",s);
    let s1 = s;
    println!("{}",s1);
    //s is invalid

    let s = String::from("hello");
    takes_ownship(s);
     // s's value moves into the function... // ... and so is no longer valid here

    let x = 5;
    makes_copy(x);
    // but i32 is Copy, so it’s okay to still
    // use x afterward


    let s1 = gives_ownship();
    let s2 = String::from("hello");
    let s3 = takes_and_give_back(s2);

    //return
    let s1 = String::from("hello");
    let (s2, len) = cal(s1);
    println!("s2={},len = {}", s2, len);
    
    //reference- immutable
    let s1 = String::from("hello");
    let len = cal_ref(&s1);
    println!("s1={},len = {}", s1, len);

    // reference mutable
    let mut s2 = String::from("hello");
    //单一指向
    change(&mut s2);

    let mut s = String::from("hello world");
    let word = first_world(&s);
    s.clear();
    println!("word = {}",word);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello={},world={}",hello,world);
}
fn first_world(s:&String) -> usize{
    //string -> bytes
    let bytes = s.as_bytes();
    //iter:array
    //enumerate wraps the result of iter and 
    //returns each element as part of a tuple instead. 
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}
fn first_world1(s:&String) -> &str{
    //string -> bytes
    let bytes = s.as_bytes();
    //iter:array
    //enumerate wraps the result of iter and 
    //returns each element as part of a tuple instead. 
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}
fn change(s:&mut String){
    s.push_str(",world.");
}
fn cal_ref(s:&String) -> usize{
    s.len()
}
fn cal(s1:String) -> (String,usize){
    let len = s1.len();
    (s1,len)
}
fn gives_ownship() -> String{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_give_back(a_string:String) -> String{
    a_string
}

fn takes_ownship(some_string:String){
    println!("{}",some_string);
}

fn makes_copy(some_integer:i32){
    println!("{}",some_integer);
}