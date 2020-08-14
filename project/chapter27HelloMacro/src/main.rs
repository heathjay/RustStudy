use chapter27HelloMacro::HelloMacro;


#[derive(HelloMa)]
struct Pancakes;
impl HelloMacro for Pancakes{
    fn hello_macro(){
        println!("hello, macro!my name is pancakes")
    }
}
fn main(){
    Pancakes::hello_macro();
}