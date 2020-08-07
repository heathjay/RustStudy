# 语句和表达式
- 语句statement，执行， let y = 3;
    let x = (let y = 0); //出现错误
- 表达式，带返回数值expression,
    - 函数调用是一个表达式，
    - 宏调用
    - 作用域{}
    let y = {
        let x = 3;
        x + 1
    }

# if
if arms{

}else{

}
- 必须是bool
- Rust 只会执行第一个条件为真的代码块，并且一旦它找到一个以后，不会检查剩下的条件下。
## - let number = if condition{
        5
    }else{
        6
    };
- let min = if a < b{
    a
}else{
    b
};
    - if 和 else 分支的值类型一定要相同

# loop
    - example
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2
        }
    };
# while 
    - while number != 0{

    }
# for
    1. 消除index超出
    let a = [10, 20, 30 , 40, 50];
    for element in a{
        println!("the element = {}", element);
    }
## Range 标准库
    for number in (1..4).rev(){
        println!("{}!", number);
    }

# Ownership
    - 运行程序都必须管理其使用计算机内存的方式。
    - 通过ownership进行内存管理，编译器在便宜的时候会根据一系列的规则，进行检查。运行时，所有权系统的任何功能都不会渐慢程序
    
## Stack 和 heap
    - 栈中的所有数据都必须占用已知且固定的大小。
    - 在编译大小未知数据或者可能变化的数据的时候，放入堆中，缺乏组织的，
    - 当你想要放入数据到heap的时候，你需要申请一个空间，操作系统在堆的某处找一个足够的空位，并把它标志成已用，并返回一个表示该位置的地址的pointer allocating on the heap.
    - 数据放入stack 不属于分配，因为指针的大小是已知并且固定的，你可以将指针存储在stack上面，但是你想要访问实际数据的时候必须通过指针，
    - 入栈快

## rule
1. RUST每一个值都有一个owner的变量
2. 值只有一个owner
3. 当所有者变量离开作用域的时候，这个值将会被抛弃
## 作用域scope
### string
    let mut s = String::from("hello ");
    s.push_str(", world!");
    println!("{}",s);
    let xs = "hello,world";//字面值
    - 内存在拥有他的变量离开作用域后，就被自动释放。drop
## 移动:moveon
    let x = 5;
    let y = x; // x y 放入stack : 只在stack进行拷贝，实现Copy trait
    let s1 = String::from("hello");
    let s2 = s1;//s1 invalid heap ; s1 不会调用drop
    // string : pointer + len + capacity
## 克隆:copy
    s.clone();//深度拷贝，heap上进行数据拷贝
    简单数据类型进行copy： int float bool char i32 u32 元组(i32, i32)
## 所有权与函数
    - 值传递可能move或者copy
    - s进入函数，不再有效，moveon
## 返回值和作用域
    - 返回值可以转移所有权
## reference // borrowing
    fn calculate_length(s : &String) -> usize{
        s.len()
    }
    // s引用:只有ptr -> s1 ptr, len, cap 
    - dereference *
    let len = calculate_length(&s);
    - 不能进行修改，defualt
    - 可变引用， &mut String
    - 特定作用域中的特定数据只能有一个可变引用，避免data race
    - 可变不可变不能同时拥有
    - 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
### 引用的规则
    1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
    2. 引用必须总是有效的

## Slice
    1. 没有所有权，允许你引用集合中一段连续的元素序列，而不引用整个集合
    fn first_word(s : &String) -> usize{
        let bytes = s.as_bytes();
        for(i, &item) in bytes.iter().enumerate(){
            if item = b' '{
                return i;
            }
        }
        s.len()
    }
    2. string slice:
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    //开始位置和长度
    //slice -> ptr + len
    fn first_world(s: &String) -> &str{
        let bytes = s.as_bytes();
        for(i, &item) in bytes.iter().enumerate(){
            if item == b' '{
                return &s[0..i];
            }
        }
        &s[..]
    }
    // word 绑定string,
    let word = first_word(&s);
    s.clear();//错误
    let s2 = "hello world";//slice
    - 数组可以当作slice

# structure/ structs
struct User{
    email:String,
    userName: String,
    active : bool,
}
let user1 = User{
    email:String::from("baidu@sina.com"),
    userName: String::from("john"),
    active: true,
};
- 变量与字段同名时可以简化初始化语法
fn build_user(email:String, username: String) -> User{
    User{
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}
- 更新结构体
let user2 = User{
    email:String::from("ss@email.com"),
    username: String::from("xx"),
    ..user1
}
- 没有命名字段的元组结构体来创建不同的类型
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
## 结构体数据的所有权
- 结构体拥有他所有的数据，整体有效，数据才有效，所以不用&str,
- 如果存储其他数据的引用，需要加上生命周期lifetime,确保结构体引用的数据有效性和结构体本身保持一致，
## 与元组类似
    let rect1 = (30, 50);
    area(rect1);
    fn area(dimensions:(u32, u32)) -> u32{
        dimensions.0 * dimensions.1
    }
## trait给结构体增加
- Display: 打印实现println！("{}",rect);
- Debug: println!("{:?}");// println!("{:#?}")
## 方法语法
impl Rect{
    fn area(&self) -> u32{}
}
### 自动引用和解引用
- 自动为object添加&, &mut 或者 * 以便object 与方法签名匹配。
等价：
    p1.distance(&p2);
    (&p1).distance(&p2);//等价

### 关联函数（不以self为参数，不作用在一个实例上，
    String::from

# 枚举和模式匹配
- 针对不同的枚举值进行不同的操作
enum IpAddrKind{
    V4(u32,u32,u32,u32),
    V6(String)，
}
- 每个成员可以处理不同类型和数量的数据, 比起结构体更灵活
- 也可以通过impl进行方法
impl Message{
    fn call(&self){

    }
}
l
## Option 枚举和相对空值的意义
1. 要么有值，要么没有
2. 不用进行引用
enum Option<T>{
    Some(T),
    None,
}
let some_num = Some(5);
let no_num: Option<i32> = None;

## Match 控制
- 一个值与一系列的模式进行比较，并根据匹配执行相应代码
match coin{
    Coint::Penny => 1,
    Coint::Nickel => 5,
}
- 绑定值的模式
- 匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值。从枚举里面提取值
#[derive(Debug)]
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
fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}, state);
            25
        },
    }
}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1)，
    }
}
- 穷尽的exhaustive
- 通配符_ 剩下所有;
- if let 只处理一个模式
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value{
        println!("three");
    }else{
        /// _ 相当于通配符
    }

# 包, Crate 和模块管理
- 一个包要包含多个二进制的Crate和一个可选的crate库
- packages: Cargo 的一个功能，他允许你构建测试和分享crate
- Crate: 一个模块的树型结构，他形成了库或者二进制项目
- 模块和use：允许你控制控制作用和路径的私有性
- path：一个命名例如结构体、函数或模块等的方式
## 包和Crate
- crate root是一个源文件，rust编译器以他为起始点，并构成crate的根模块
- 包是提供一系列功能的一个或者多个crate,一个包可能会包含一个Cargo.toml,阐述如何构建这些Crate
- 一个包只能一个library crate 和 多个binary crate； 至少一个crate
- src/main.rs 就是一个与包同名的二进制的crate的crate根，
- src/lib.rs 同理
- 通过将文件放在src/bin目录下，一个包可以拥有多个二进制的crate，每一个文件都会被编译成一个独立的二进制crate，
## 模块来控制作用域和私有性
- 模块能将一个crate中的代码进行分组，控制私有性
crate
└── front_of_house
    ├── hosting
    │
        ├── add_to_waitlist
    │
        └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment

## 路径
- 绝对路径，crate根开始
- 相对路径：self 和super
- 默认私有，父模块不能使用子模块私有的，但子模块可以使用父模块私有

## 公有结构体和枚举
- 结构体定义pub,但内容不，
- 枚举如果公有，则整体公有。

## use引入作用域
- 函数的父模块进行引入，父模块：：fn
- 用as提供新的名称
- 重导出： 结合pub和use 让调用你编写的代码的代码能够像在自己的作用域引用这些类型，
## 使用外部包: Cargo.toml -> dependencies + use
crates.io 上面可以用
标准库也是外部包，但不需要通过use
- use std::{comp::Order, io};
//同时使用多个包
use std::io::{self, Write};
use std::*;//引入所有公有项目

# 常见集合
- vec:一个挨着一个进行存储
- hash
- string
## vec
1. 索引值直接访问数，或者用get返回一个Option
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];    //如果没有会panic
println!("The third element is {}", third);
match v.get(2) {
    Some(third) => println!("The element is {}", third);//处理Some(&element)
    None => println!("there is no third element."); // 如果没有会none
}
//下面会报错：不能在相同的作用域中存在可变和不可变的两种引用
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
v.push(6);
println!("The first element is: {}", first);

// 遍历不可变Vec
let v = vec![100, 32, 57];
for i in &v{
    println!("{}",i);
}

//遍历可变Vec
let mut v = vec![100, 32 , 55];
for i in &mut v{
    *i += 58;//修改可变引用所指向的值，使用 *
}

### 枚举加上vec存储不同类型
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
//vec需要知道多少内存

## String 字符串
### 新建 
let mut s = String::new();
let data = "initial contentx";
let s = data.to_string();
### 更新字符串
1. push_str 和 push 附加字符串
let mut s = String::from("foo");
s.push_str("bar");//不需要所有权
//通过push_str来附加字符串slice,从而使String变长。
s.push('l');//加入一个字母
2. format!拼接字符串
let s1 = String::from("Hello ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
fn add(self, s:&str) -> String{}
//s1把所有权给了s3
//string强制转换成了&str
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
3. 内部表现
- String是一个vec<u8>的封装，
- 索引字符串通常是一个怀点子，因为字符串索引应该返回的类型是不明确的：字节值、字符、字形簇或者字符串slice
- 遍历:
for c in s.chars(){
    println!("{}",c);
}
for b in s.bytes(){
    println!("{}“，b);
}
## Map 存储键值对
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 20);
//zio方法茶ungjian一个元组
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
//collect 很多不同的数据结构，所以需要显性

### 组成新的map包括所有权移交
let field_name = String::from("ff");
let field_value = String::from("good");
let mut map = HashMap::new();
map.insert(field_name, field_value);
let team_name = String::from("ff");
let v = map.get(&team_name);
//返回一个Option<V>的值
for (key, value) in &map{
    println!("{}:{}",key,value);
}
### 更新hashmap
- 覆盖一个值
let mut scores = HashMap::new();
scores.insert(String::from("b"), 10);
scores.insert(String::from("b"), 25);
println!("{:?}",scores);
- 只在没有值的时候插入
let mut scores = HashMap::new();
scores.insert(String::from("b"), 10);
scores.entry(String::from("y")).or_insert(50);
scores.entry(String::from("b")).or_insert(2);
println!("{:?}", scores);
- 统计单词出现次数
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in map.split_whitespace(){
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

# 错误处理
- 可恢复错误recoverable ： Result<T, E>
- 不可恢复错误unrecoverable: panic!
    - bug的同义词
## panic! 不可恢复错误
- 打印一个错误信息，展开并清理栈数据，接着退出。
- 通过Cargo.toml 的 [profile] 增加 panic = 'abort';
- backtrace 使用，
## Result 与可恢复的错误
enum Result<T, E>{
    Ok(T),
    Err(E),
}
let f = File::open("hello.txt");
let f = match f{
    Ok(file) => file,
    Err(error) => {
        panic!();
    },
};
## 失败时panic简写unwrap和expect
let f = File::open("hello.txt").unwrap();//默认
let f = File::open("hello.txt").expect("failed to open hello.txt");

## 传播错误
1. 当编写一个其实会调用一些可能会失败的操作的函数的时候，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理，这称之为传播错误，
use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error>{
    let f = File::open("hello.txt");

    let mut f = match f{
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
## 传播错误的简写：？运算符
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}