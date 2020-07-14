struct User{
    username : String,
    email: String,
    sign_in_count : u64,
    active : bool,
}
#[derive(Debug)]
struct Rec{
    width:u32,
    height:u32,
}   
fn main() {


    let user = User{
        email : String::from("www.baidu.com"),
        username : String::from("jc"),
        sign_in_count : 22,
        active : true,
    };
    println!("user.email={}",user.email);


    let mut user1 = User{
        email : String::from("www.sina.com"),
        username : String::from("jccc"),
        sign_in_count : 1,
        active : true,
    };
    user1.email = String::from("www.qq.com");
    println!("user1.email={}",user1.email);

    let user2 = User{
        email: String::from("another@example.com"), 
        username: String::from("anotherusername567"),
        //用..剩下的属性被user1替代
        ..user1
    };

    //tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0,0,0);


    //example
    let rec1 = (30, 50);
    println!("area is {}",area(rec1));

    let rec2 = Rec{
        width : 3,
        height : 22,
    };
    /*
    println!("area is {}", cal_area(rec2));
    //ownership moves
    println!("width:{}",rec2.width);
    */
    println!("area is {}", cal_area(&rec2));

    //debuging
    println!("rec2={:?}",rec2);
    println!("rec2={:#?}",rec2);


    //methods
    println!("area is {}", rec2.cal_area());

    rec2.print();
    //more parameters
    let rect1 = Rec { width: 30, height: 50 }; 
    let rect2 = Rec { width: 10, height: 40 }; 
    let rect3 = Rec { width: 60, height: 45 };
    println!("rec1 can hold rect2 :{}",rect1.can_hold(&rect2));
    println!("rec1 can hold rect3 :{}",rect1.can_hold(&rect3));
    let rect4 = Rec::square(32);
}

impl Rec{
    fn cal_area(&self) -> u32{
        self.width * self.height
    }  
    fn print(&self){
        println!("rec:width = {} height = {}",self.width,self.height);
    }

    fn can_hold(&self,other : &Rec) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size : u32) -> Rec{
        Rec{
            width : size,
            height : size,
        }
    }
}
fn cal_area(rec:&Rec) -> u32{
    rec.width * rec.height
}
fn area(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}


fn build_user(username:String, email:String) -> User{
    User{
        email:email,
        username : username,
        active : true,
        sign_in_count : 1,
    }
}
