use chapter21OBP::Draw;
use chapter21OBP::{Button, Screen};
struct SelectBox{
    width: u32,
    height: u32,
    option: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        //
    }
}

fn main(){
    let screen = Screen{
        components: vec![
            Box::new(SelectBox{
                width:75,
                height:10,
                option: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button{
                width: 50,
                height: 10,
                label:String::from("Ok"),
            })
        ],
    };
    screen.run();
}