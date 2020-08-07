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
## 何时panic？
- 示例、代码原型和测试都非常适合panic

# 范型\trait和生命周期
## generics
- 具体类型或者其他属性的抽象替代，
### 函数定义
- 函数名称和参数列表之间的尖括号<>
fn largest<T>(list:&[T]) -> T //T 类型的slice
### 结构体中泛型定义
struct Point<T>{

}
### 方法泛型定义
impl<T> Point<T>{}
- 必须在impl之后声明一个T,这样就可以在point<T>上实现的方法中引用他，rust知道point的尖括号里面的类型是泛型而不是具体的值类型
### 枚举泛型
enum Option<T>{}
## trait
- 定义范型行为的方法，可以结合来将泛型限制为拥有特定行为的类型，而不是任意类型
- 定义共享的行为：特定类型可能拥有与其他类型共享的功能
pub trait Summary{
    fn summarize(&self) -> String;
}
- 一行一个方法名并且以分号结尾
impl Summary for name_struct{}
- 限制：
    - 只有当trait或者要实现trait的类型位于crate的本地作用域时，才能为该类型实现trait
- 可以有默认实现:进行保留或者重载
### trait 作为参数
- 定义一个函数调用其参数上的某个trait定义的方法
pub fn notify(item: impl Summary){
    println!("{}", item.summarize());
}
- 可以是任何类型，只要实现了trait，多态
- 可以结合范型, T: trait
pub notify<T: Summary>(item: T){

}
- 多个trait实现
pub fn notify(item: impl Summary + Display)
pub fn notify<T: Summary+Dislay>(item:T)
    - 加入where可以进行简化
pub fn some_function<T: Display + Clone, U: Clone + Debug>(t:T, u: U) -> i32
pub fn some_function<T,U>(t:T, u:U) ->i32 where T: Display+Clone,
                                                U: Clone + Debug
### trait bound 有条件实现方法

### 返回实现trait类型
fn return_function() -> impl Summary{}

## lifetimes:
- 向编译器提供引用如何相互关联的泛型
- 允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性
- 有一个借用检查器
    - 定义函数的时候，并不知道传递给函数的具体值，所以也不知道到底是if还是else会被执行，如果输入没有生命周期，rust无法判断
### 生命周期注解语法
- 生命周期注解并不改变任何引用的生命周期的长短，与当函数生命中指定了泛型类型参数后就可以接受任何类型一样，当指定了泛型生命周期后函数也能接受任何生命周期的引用
- 描述了多个引用生命周期互相的关系，而不影响其生命周期
- '进行影响，放在&之后
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
- 防止悬挂
### 结构体定义的生命周期注解
- 我们将定义包含引用的结构体，不过者需要为结构体定义中的每一个引用添加生命周期注解。
struct ImportantExcerpt<'a>{
    part: &'a str,
}
fn main(){
    let novel = String::from("call me ismmm");
    let first_sentence = novel.split('.')
        .next()
        .expect("dould not find a ");
        let i = ImportantExcerpt{part: first_sentence};
}
### 生命周期省略规则
- 编译器会考虑，如果代码符合这些规则，就无需明确指定生命周期注解
1. 输入生命周期
    - 每一个引用的参数都有他自己的生命周期参数，两个就有两个
2. 输出生命周期
    - 只有一个输入生命周期参数，那么他被赋予所有输出生命周期参数
    - 如果方法有多个输入生命周期并且其中一个参数是&self 或者&mut self 说明这是一个对象的方法。那么所有输出生命周期参数被赋予self的生命周期

### 方法定义中的生命周期注解
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
- impl 之后和类型米ingcheng之后的生命周期参数是必要的，不过因为第一条生命周期规则我们并不必须标注self引用的生命周期
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
### 静态生命周期 'static
存活整个程序期间,
### 联合trait、生命周期、泛型
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display{
        println!("jsisjis");
        if x.len() > y.len(){
            x
        }else{
            y
        }
}

# 编写测试程序cargo test
1. 设置任何所需要的数据或者状态
2. 运行需要测试的代码
3. 断言其结果是我们所期望的

test属性，宏方法，should_panic属性
- 新建一个库项目，她会自动生成一个测试模块和一个检测函数
- cargo new adder --lib
#[cfg(test)]
mod tests{
    #[test]
    fn it_works(){
        assert_eq!(2+2,4);
    }
}
- cargo test
## assert!宏来检查结果
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        slef.width > otehr.width && self.height > other.height
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn larget_can_hold_smaller(){
        let larger = Rectangle{width:8, height : 7};
        let smaller = Rectangle{width: 5, height : 1};
        assert!(larger.can_hold(&smaller));
    }
}
## assert_eq!和 assert_ne!
- assert!可以打印更加有用的信息
assert!(
    result.contains("Carol"),
    "Greeting did not contain name, value was `{}`", result
);

## should_panic 检查panic
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100(){
        Guess::new(200);
    }
}
## Result<T,E> 用于测试
#[cfg(test)]
mod tests{
    #[test]
    fn it_works() -> Result<(),String>{
        if 2 + 2 == 4{
            Ok(())
        }else{
            Err(String::from("two plus two does not equal four"))
        }
    }
}
- 不能对这些使用Result的测试使用should_panic注解，
## 控制test
1. 运行多个测试时，Rust默认使用线程来并行运行，这意味着测试会更快地运行完成。
cargo test -- --test-threads=1
2. 显示输出
cargo test -- --nocapture
3. 指定名字来运行部分测试
- 过滤精确： cargo test 全名
- 部分过滤: cargo test 公有
4. 忽略某些测试
- #[ignore]
- 希望只运行一个ignore cargo test -- --ignored

## 测试的组织结构
1. 单元测试
    - 隔离环境，测试一个模块
    - 每个文件中创建包含测试函数的tests模块，并使得cfg(test)标注模块

2. 集成测试
- 集成测试对于你需要测试的库完全是外部的，同其他使用库的代码一样使用库文件。
- 需要创建一个tests目录，新建一个文件
- 需要在文件顶部添加use 原目录下的crate
- 不需要将任何一个代码标注为#[cfg(test)]，
- Cargo test --test 某个继承测试文件中的所有测试

3. 集成测试中的子模块
    - 每一个集成测试文件都是一个单独的crate
    - 不让一个common出现在测试结果中，建立一个tests/common/mod.rs文件
    - tests目录中的子目录不会被当作单独的crate编译或作为一个测试结果部分出现在测试输出中
    - 一旦拥有了tests/common/mod.rs，就可以将其作为模块以便在任何集成测试文件中使用，
    use adder;
    mod common;
    #[test]
    fn it_adds_two() {
        common::setup();
        assert_eq!(4, adder::add_two(2));
    }