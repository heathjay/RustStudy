/*
//暂时不能编译，因为没有大小
enum List{
    Cons(i32, Box<List>),
    Nil,
}
*/
use std::rc::Rc;
enum List{
    Cons(i32, Rc<List>),
    Nil
}
use std::ops::Deref;
//创建一个智能指针
struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}
//drop trait
struct CustomSmartPointer{
    data: String,
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}


use crate::List::{Cons, Nil};

fn main() {
    //box
    //指向堆的指针
    //单个数据在不常见
    let b = Box::new(5);
    println!("b={}",b);


/*
    //cons list
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1,
        Box::new(Cons(2,
        Box::new(Cons(3,
        Box::new(Cons(4,
        Box::new(Nil)))))))
    );

    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);

*/
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);

    //解强制引用多态
    let m = MyBox::new(String::from("hello"));
    hello(&m);//hello(&(*m)[..]);


    let c = CustomSmartPointer{data:String::from("my stuff")};
    let d = CustomSmartPointer{data:String::from("other stuff")};
    println!("customSmartPointers created.");

/*
    let a = Cons(5,
        Box::new(Cons(10,
        Box::new(Nil)))
    );

    let b = Cons(3, Box::new(a));
    //Cons成员拥有其存储的数据，所以当创建b列表时，a被移动到了b这样b就拥有a
    //可以改变Cons的定义来存放一个引用，不过接着必须指定生命周期参数。通过指定生命周期参数，表明列表中的每一个元素都至少与列表本身存在的一样久。
    let c = Cons(4,Box::new(a));
*/


    let a = Rc::new(Cons(4, Rc::new(Cons(10,Rc::new(Nil)))));
    println!("count a= {}",Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count b= {}",Rc::strong_count(&a));
    let c = Cons(4,Rc::clone(&a));
    println!("count c= {}",Rc::strong_count(&a));
}
fn hello(name : &str){
   println!("hello,{}",name) ;
}

