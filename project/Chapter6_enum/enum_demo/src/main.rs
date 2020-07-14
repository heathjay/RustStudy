struct IPAddr{
    kind:IpAddrKind,
    address:String,
}

enum IpAddrKind{
    V4,
    V6,
}
/*
enum IpAddrKind{
    V4(String),
    V6(String), //let home = IpAddrKind::V4(String::from("127,0,0,1"));
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

 struct Ipv4Addr { 
     // --snip--
}
struct Ipv6Addr { 
    // --snip--
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

 struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32, }
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    } 
}
    let m = Message::Write(String::from("hello")); 
    m.call();
*/

/*
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
//match_pather
fn value_in_cents(coin : Coin) -> u32{
    match coin{
        Coin::Penny => {
            println!("good job!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
*/
#[derive(Debug)]//{:?}
enum UsState{
    Alabama,
    Alaska,
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u32 { 
    match coin {
    Coin::Penny => 1, 
    Coin::Nickel => 5,
    Coin::Dime => 10, 
    Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }, 
    }
}
// enum Option<T>{
//     Some(T),
//     None,
// }
/*
fn plus_one(option:Option<i32>) ->Option<i32>{
    match option{
        Some(i) => Some(i+1),
        None => None,
    }
}
*/
fn plus_one(x: Option<i32>) -> Option<i32> { 
    match x {
     None => None,
     Some(i) => {
         println!("some(i)={}",i);
         Some(i + 1)
     },
   
    }
}
fn main() {

    //same type
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home =IPAddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };

    let rout = IPAddr{
        kind:IpAddrKind::V6,
        address:String::from("::0"),
    };
    let c = Coin::Penny;
    let num = value_in_cents(&c);
    println!("num = {}",num);

    let coint = Coin::Quarter(UsState::Alabama);
    let num = value_in_cents(&coint);

    //option + match
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //占位符
    let some_u8_value = 0u8; 
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"), 
        5 => println!("five"), 
        7 => println!("seven"),
         _ => (),
    }


    
     let some_u8_value = Some(0u8); 
    /*
     match some_u8_value {
        Some(3) => println!("three"),
        _ => (), 
    }
    */
    //if let
    if let Some(3) = some_u8_value { 
        println!("three");
    }


    /*
     let mut count = 0; 
     match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1, }
    */
        //if let else
    let mut count = 0;
    if let Coin::Quarter(state) = coint{
        println!("state quarter from {:?}",state);
    }else{
        count+=1;
    }
}

