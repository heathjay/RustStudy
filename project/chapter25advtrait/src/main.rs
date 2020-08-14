use std::ops::Add;
#[derive(Debug, PartialEq)]
struct Point{
    x : i32,
    y : i32,
}

impl Add for Point{
    type Output = Point;
    fn add(self, other:Point) -> Point{
        Point{
            x:self.x + other.x,
            y: self.y + other.y,
        }
    }
}


//调用相同名称的方法
trait Pilot{
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}

struct Human;
impl Pilot for Human{
    fn fly(&self){
        println!("this is your captain speaking");
    }
}

impl Wizard for Human{
    fn fly(&self){
        println!("Up!");
    }
}

impl Human{
    fn fly(&self){
        println!("*waving arms furiously*");
    }
}

//完全限定语法
//<type as trait>::function
trait Animal{
    fn baby_name() -> String;
}

struct Dog;

impl Dog{
    fn baby_name() -> String{
        String::from("spot")
    }
}

impl Animal for Dog{
    fn baby_name() -> String{
        String::from("pupy")
    }
}
//父类trait
use std::fmt;
trait OutlinePrint: fmt::Display{
    fn outline_print(&self){
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len +4));
            println!("*{}*", " ".repeat(len+2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({},{})",self.x,self.y)
    }
}
impl OutlinePrint for Point{}


//newtype
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f, "[{}]",self.0.join(", "))
    }
}


fn main() {
    assert_eq!(Point{x : 1, y : 0} + Point{x:2, y:3}, Point{x:3,y:3});

    //直接调用
    //直接调用实现在human的fly方法
    let person = Human;
    person.fly();

    //用更明显的方法调用。
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("a bady dog is called a {}", Dog::baby_name());
    /*
    rust并不知道该使用哪一个实现
    因为是关联函数而不是方法，没有self参数，会有编译错误
    所以使用完全限定语法
    println!("a bady dog is called a {}", Animal::baby_name());
    */
    println!("a bady dog is called a {}", <Dog as Animal>::baby_name());


    //newtype
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
