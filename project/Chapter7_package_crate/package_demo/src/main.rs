/**
 * module
 *  - control / organize codes
 * 
*/
mod sound;

//pub with struct and enum
mod plant{
    pub struct Vegetable{
        pub name:String,
        id:i32,
    }

    impl Vegetable{
        //slice for string literals
        pub fn new(name:&str) ->Vegetable{
            Vegetable{
                name:String::from(name),
                id:1,
            }
        }
    }
}
//pub enum
mod menu{
    pub enum App{
        Soup,
        Salad,
    }
}
//use - self
use plant::Vegetable;
//use self::plant::Vegetable;

//use-pub
mod perform{
    pub use crate::sound::instrument::woodwind;
    pub fn per(){
        woodwind::clarinet();
        woodwind::clarinet(); 
    }

}
fn main() {
     //abs path
     crate::sound::instrument::woodwind::clarinet();
     //rel
     sound::instrument::woodwind::clarinet();

    let mut v = plant::Vegetable::new("squash");
    let mut v1 = plant::Vegetable::new("ss");
    v.name = String::from("jjccpp");
    println!("v.name:{}",v.name);
    let v2 = Vegetable::new("badss");
    let ordre1 = menu::App::Soup;
    //pub+use
    perform::woodwind::clarinet();
    perform::per();

}
