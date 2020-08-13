struct Point{
    x : i32,
    y : i32,
}
enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(Color),
}

enum Color{
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
}
fn main() {
    let x = Some(5);
    let y = 10;
    //
//     matched, y = 5
// at the end x= Some(5), y = 10
    match x{
        Some(50) => println!("got 50"),
        Some(y) => println!("matched, y = {:?}",y),
        _ => println!("default case x ={:?}",x),
    }
    println!("at the end x= {:?}, y = {:?}",x,y);

    let x = 1;
    match x{
        1|2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x{
        1..=5 =>println!("one through five"),
        _=>println!("something else"),
    }
    //..= 数字 char
    let x = 'c';
    match x{
        'a'..='j' => println!("early ascii letter"),
        _=>println!("anything"),
    }

    //解构结构体
    let p = Point{x:0,y:8};
    let Point{x:a,y:b} = p;
    assert_eq!(0,a);
    assert_eq!(8,b);
    let Point{x,y}= p;
    //部分结构
    match p{
        Point{x, y : 0} => println!("on the x axis at {}",x),
        Point{x : 0,y} => println!("on the y axis at {}", y),
        Point{x,y} => println!("on neither axis:{} {}", x, y),
    }
    /*
    let msg = Message::ChangeColor(0,160,255);
    match msg{
        Message::Quit =>{
            println!("quit");
        }
        Message::Move{x,y} =>{
            println!("move :{} {}", x,y);
        }
        Message::Write(text) => println!("text:{}",text),
        Message::ChangeColor(r,g,b) => println!("r,g,b ={} {} {}",r,g,b),
    }
    */
    let msg = Message::ChangeColor(Color::Hsv(0,160,255));
    match msg{
        Message::ChangeColor(Color::Hsv(h,s,v))=>println!("hsv:{},{},{}",h,s,v),
        Message::ChangeColor(Color::Rgb(r,g,b)) => println!("rgb:{},{},{}",r,g,b),
        _ => ()
    }
    

    //用_忽略部分
    let mut setting = Some(5);
    let new_setting = Some(10);
    match (setting, new_setting){
        (Some(_),Some(_))=>
        println!("Can't overwrite an existing customized value"),
        //两者有一个为none就
        _=>setting = new_setting,
    }
    println!("setting is {:?}",setting);
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }
    //以_为开头
    let s = Some(String::from("hello"));
    /*
    if let Some(_s) = s{
        println!("found a string");
    }
    //s的值仍然会移动到_s，并阻止我们再次使用s，然后只使用_，并不会绑定值
    //println!("{:?}",s);
    */

    if let Some(_) = s{
        println!("found a string");
    }
    println!("{:?}",s);

    //..省略
    let numbers = (2, 4, 8, 16, 32);
    match numbers{
        (first,..,last) => println!("some numbers:{}, {}",first, last),
    }

    //匹配守卫match guard
    let num = Some(4);
    match num{
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}",x),
        None => ()
    }
    //@绑定
    //绑定值的同时测试它
    enum mx{
        Hello{id:i32}
    }
    let msg = mx::Hello{id:5};
    match msg{
        mx::Hello{id: id_value @ 3..=7} => println!("found an id in range : {}", id_value),
        mx::Hello{id: 10..=12} => println!("found an id in another range"),
        mx::Hello{id} => println!("found some otehr id:{}",id),
    }
}
