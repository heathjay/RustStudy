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
# I/O项目
- 构建一个与文件和命令行输入/输出交互的命令行工具来练习
- 运行速度，安全性，单二进制文件输出和跨平台
- 一个自己的grep项目
1. 让项目能够接受命令行参数：文件名和要搜索的字符串： cargo run searchstring example-filename.txt
- 需要一个std::env::args 返回一个传递给程序的命令行参数的迭代其
- 生成一系列值，可以在迭代器上调用collect方法将其转换成一个集合
- args():无效的参数会panic，如果需要无效args_os代替
2. 读取文件
- fs::read_to_string
3. 重构改进模块性和错误处理
- 将程序拆成main.rs和lib.rs并将程序的逻辑放进lib.rs中，
- 当命令行解析逻辑比较小时，可以保留在main.rs中。
- 当命令行解析变得复杂的时候，提取到lib中
- main函数的主要责任应该被限制：
    - 使用参数值调用命令行解析逻辑
    - 设置任何其他的配置
    - 调用lib.rs中的run函数
    - 如果run返回错误，就处理这个错误
- main程序运行，lib真正的业务逻辑
- 进行简单的函数剥离
- 进行结构体整合，注意所有权问题，尽量少用clone()
- 建立一个config的new函数
- 错误信息进行修复:panic,Result
    - unwrap_or_else定义在标准库Result<T, E>中，如果是ok类似于unwrap：返回ok内部封装的值
    - 如果是err时，他会调用一个闭包closure，也就是一个我们定义的作为参数传递给unwrap_or_else的匿名函数，参数:|err|
- 从main中提取逻辑
    - 提取run函数。
    - 处理run的错误信息，返回一个result，使用?
    - main函数中用if let去查看是否返回一个err
- 将代码拆分到库crate中
    - 将不是main函数的代码从src/main.rs移动到新文件src/lib.rs中：
    - run函数定义
    - 相关的use语句
    - Config的定义
    - Config::new函数定义

- 采用测试驱动开发完善库的功能
    - 已经把逻辑提取到src/lib.rs并将所有的参数解析和错误处理留在src/main.rs中
    - 为代码的核心功能编写测试将更加容易
    - 遵循TDD模式来增加搜索逻辑
    1. 编写一个会失败的测试，并运行它以确保其因为你期待的原因失败
    2. 编写或修改刚好足够的代码来使得新的测试通过
    3. 重构刚刚增加或者修改的代码，并确保测试仍然能通过
    4. 从步骤1开始

- 编写使测试通过的代码
    - 遍历内容的每一行文本
        - lines方法遍历每一行
    - 查看这一行是否包含要搜索的字符串
        - 检查当前行是否包含查询字符串的功能，
    - 如果有，将这一行加入列表返回值中
    - 如果没有，什么也不做
    - 返回匹配到的结果列表
- 处理环境变量
    - 用户可以通过设置环境变量来设置搜索是否是大小敏感的，
    - 允许用户设置环境比那两一次后在整个终端会话中所有的搜索都将是大小写不敏感的
    - 编写一个大小写不敏感search函数的失败测试
    - env::var("xx").is_err();//会返回false并进行大小写不敏感
- 将错误信息输出到标准错误而不是标准输出
    - 目前为止，我们将所有的输出都println!到了终端，大部分终端都提供了两种输出：
        - 标准输出 standard output, stdout 
        - 标准错误 standard error, stderr： 则用于错误信息，
    - 这种区别允许用户选择将程序正常输出定向到一个文件中并仍将错误信息打印到屏幕上。
- 检查错误u应该写入何处
    - 可以通过将标准输出流重定向到一个文件同时产生一个错误来做到这点
    - 用eprintln！替换println！

# 迭代器和闭包
## 闭包closure
    - 一个可以存储在变量里的类似函数结构
    - 可以捕获环境的匿名函数
    - 是一个保存变量或作为参数传递给其他函数的匿名函数，可以在一个地方创建闭包。然后在不同的上下文中执行闭包运算
    - 允许捕获者作用域中的值，
    - 函数重构-我们不希望在完全无需其结果的情况下调用函数，不过仍然希望只调用函数一次
    - 希望能够在程序的一个位置指定某些代码并只在程序的某处实际需要结果的时候执行这些代码。
    - 重构使用闭包存储代码：
        - 我们可以定义一个闭包并将其存储在变量里面，把整个函数放入到闭包中：
let expensive_closure = |num|{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
    - 闭包定义是赋值符号=后面的部分，定义将||开始，指定闭包参数，可以用逗号隔开
    = |num, para2|
    - 存放闭包的大括号，如果只有一行就可以省略，最后需要；let
    {};
    - 最后一行作为返回值，匿名函数的定义，而不是调用匿名函数的返回值，
    - 使用闭包的原因是我们需要在一个位置定义函数，存储代码，并且在之后实际使用的时候进行调用。
    - 一个if中调用了两次闭包，可以在if中创建一个本地变量存放闭包调用的结果来解决这个问题，有另一个解决方案
### 闭包类型推断和注解
    - 不需要在参数和返回值上著名类型，
    - 他们存储在变量中被使用，不用命名他们或者暴露给库用户调用
    - 编译器能可靠的推断参数和返回值的类型，类似于他如何推断大部分变量的类型
    - 可以进行类型注解，类似于函数
fn add_one_v1 (x: u32) -> u32{x + 1};
let add_one_v2 = |x : u32| -> u32{x+1};
let add_one_v3 =|x|{x+1};
let add_one_v4 = |x| x+1;
    - 闭包定义会为每个参数和返回值推断一个具体类型

### 使用带有泛型和fn trait的闭包
    - 可以创建一个存放闭包和调用闭包结果的结构体
    - 该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。memorization
    - 需要指定闭包类型，每个闭包实例需要独有匿名类型，
    - Fn系列列：Fn(u32) -> u32
    - Cacher::new函数获取一个泛型参数T，他定义于impl返回一个在calculation字段中存放了指定闭包和在value字段中存放了None值的Cacher实例，因为我们还没执行闭包，
    - 当调用代码需要闭包的执行结果，不同于直接调用，调用value方法，这个方法会检查self.value是否以ing有了一个some结果值，如果有，他返回some中的值并不会再次执行闭包
impl<T> Cacher<T>{
    fn new(calculation:T)->Cacher<T>{
        Cacher{
            calculation,
            value:None,
        }
    }

    fn value(&mut self, arg:u32) ->u32{
        match self.value{
            Some(v) => v,
            None =>{
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
- 慢计算只用一次
- 值缓存是一种更加广泛的实用行为，我们可以希望在代码中的其他闭包中也是用他们，然后有两个问题
    - Cacher实例假设对于value方法的任何arg参数值总会返回相同的值，这个Cacher的测试会失败:
    #[test]
    fn call_with_different_values(){
        let mut c = Cacher::new(|a|a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2,2);
    }
    - 可以用map来解决key-arg，value-值
    - 第二个问题是他的应用被限制为只能接受获取一个u32值并且返回一个u32的闭包，可以引入更多泛型
### 闭包会捕获其环境
- 函数没有的功能，他们可以捕获其环境并访问其被定义的作用域的变量
    let x = 4;
    let equal_to_x = |z| z==x;
    let y  = 4;
    assert!(equal_to_x(y));
- 当闭包从环境中捕获一个值，闭包会在闭包体中存储这个值来使用，这会使用内存并产生额外的开发，在更一般的场景中，当我们不需要闭包来捕获环境时。我们不希望产生这些开销，
- 闭包可以通过三种方式来捕获环境：
    - 获取所有权，用move
    let equal_to_x = move|z| z==x;
    - 可变借用 
    - 不可变借用
    - FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其环境，environment为了消费捕获到的变量， 闭包必须获取其所有权并在定义闭包的时候将他移动到闭包里面，其名称的Onve部分代表了闭包不能多次获取相同变量的所有权的事实，所以他只能被调用一次。
    - FnMut 获取可变的借用值所以可以改变环境
    - fn从其变量获取不可变的借用值
    - 由于所有闭包都实现了FnOnce，那些没有移动被捕获变量的所有权倒闭包内也实现了FnMUT，而不需要对被捕获的变量进行可变访问的闭包则也实现了Fn
## 迭代器iterators
    - 一种处理元素序列的方式
    - 存储稍后要执行的闭包实例，
    - 允许你对一个项目的序列进行某些处理，迭代器iterator负责遍历序列中的每一项和决定序列何时结束的逻辑。
    - 是惰性的，这意味着在调用方法使用迭代器之前他都不会有效果。
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter{
        println!("Got:{}", val);
    }
### iterator trait 和 next 方法
pub trait Iterator{
    type Item;  // 定义了关联的类型，定义了一个item类型，这个item类型作为next方法返回值类型，
    //next方法是唯一方法
    fn next(&mut self)-> Option<Self::Item>

}

### 消费迭代器的方法
- 调用next方法的方法被称为消费适配器，consuming adaptors。因为调用他们会消耗迭代器，
- 产生迭代器的其他方法，
    - iterator trait 定义了另一种方法，被称为迭代器适配器，他们允许我们将当前迭代器变成不同类型的迭代器，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果
    - 会需要消费迭代器
- 使用闭包获取环境
    - 通过使用filter迭代器适配器和捕获环境的闭包的常规用例。
    - 迭代器的filter方法获取一个使用迭代器的每一个项并返回布尔值的闭包，如果返回true，其值将会包含在filter提供的新迭代器中，如果闭包返回false，其值不会包含在结果迭代器中


### 实现Iterator trait来创建自定义迭代器
- 唯一要求就是提供next方法

## 使用迭代器并去掉clone


## 迭代器是Rust的零成本抽象zero-cost abstractions


# 进一步认识cargo 和 crate.io
## 采用发布配置自定义构建
- 在rust发布配置是预定义的、可定制的带有不同选项的配置
- cargo build 采用dev配置和运行cargo build --release 的release配置，dev配置被定义为开发时的好的默认配置，release配置则有良好的发布构建的默认配置
- 当cargo.toml文件中没有任何[profile.*]部分的时候，cargo会对每一个配置使用默认设置，通过增加爱任何希望定制的配置对应的profile.*部分，我们可以选择覆盖任意默认设置的子集。默认:
[profile.dev]
opt-level = 0
[profile.release]
opt-level = 3

0-3 优化级别，发布模式用更长的便一时间换取运行更快的代码3
## 将crate发布到crates.io
- 编写有用的文档注释
生成html文件。展示api文档
///markdown
///# example
///
///
///```
///
///let arg = 5;
///let answer = my_crate::add_one(arg);;
///assert_eq!(4,answer);
///```
///
pub fn add_one(x:i32)->i32{
    x+1
}
- cargo doc 生成：rustdoc
- panics!  /  errors /  safety

### 文档注释作为测试
- 增加示例代码块是一个清除表明怎么使用库方法。
- cargo test也会向测试那样运行文档中的代码
### 注释包含项的结构
//! 这为包含注释的项，而不是位于注释之后的项增加文档，通常用于crate根文件src/lib
文件名src/lib.rs
//! # My Crate
//!
//! `my_crate` 是一�使得��计算更�便的
//! 工具集合

### 使用pub use 导出和时的公有api
- 多层次的分层结构，不过这对于用户来说不方便，
use my_crate::some_module::another_module::UsefulType;
use my_crate::UsefulType;

- 可以使用pub use 重新导出re-export项来使公有结构不同于私有结构，重导获取位于一个位置的公有项目并将其公开到另一个位置。
//! # Art
//!
//! 一个描述美术信息的库
pub mod kinds {
/// 采用 RGB 色彩模式的主要颜色
pub enum PrimaryColor {
Red,
Yellow,
Blue,
}
/// 采用 RGB 次要颜色
pub enum SecondaryColor {
Orange,
Green,
Purple,
}
}
pub mod utils {
use crate::kinds::*;
/// 等量的混合两个主要颜色
/// 来创建一个次要颜色。
pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
// --snip--
}
}

main.rs
use art::kinds::PrimaryColor;
use art::utils::mix;
fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

//不方便：
增加到lib.rs
//!#Art
//!
//!一个描述美术信息的库
pub use self::kinds::PrimaryColor;
...

在mian.rs
use art::PrimaryColor

## 注册一个crates.io账号
cargo login
- 发布前需要在cargo.toml文件的[package]部分增加一些本crate的元信息metadata
- crate 需要一个唯一的名称。先到显得
[package]
name = "first_wins"
license = "MIT" // 必须要有一个license表示，或者license-file Apache-2.0
version =
authors =
edition = 
description = 

## 发布时永久性的，不能删除和覆盖
cargo publish
- 可以进行撤回cargo yank - 阻止被人引用。

# Cargo工作空间
- 构建一个包含二进制crate和库crate包，随着项目开发的深入，库crate持续增大，希望将其进一步拆分成多个库crate，对于这个，提供了工作空间的功能
- 帮助开发管理多个协同开发的包

## 创建工作空间
- 工作空间是一系列共享同样的Cargo.lock和输出目录的包。
- 举例：我们工作空间有一个二进制项目和两个库，二进制项目会提供主要功能，并会依赖另外两个库。
- 一个提供add_one方法而第二个会提供add_two方法，这三个crate将会是相同工作空间的一部分。
mkdir add
cd add
创建cargo.toml文件，会以[workspace]部分作为开始，并通过指定adder的路径来为工作空间增加成员。
[workspace]
members = [
    "adder",
]
接下来，在add目录下运行cargo new新建adder二进制crate:
cargo new adder
|
|----Cargo.lock
|----Cargo.toml
|----adder
|       |
|       |--Cargo.toml
|       |--src
|           |---main.rs
|----target
adder没有自己的target目录，在顶级空间上有，进入adder运行cargo build 构建也会在add/target中而不是add/adder/target
- 在工作空间中创建第二个crate
[workspace]
members=[
    "adder",
    "add-one",
]
cargo new add-one --lib
文件名: add-one/src/lib.rs
pub fn add_one(x:i32)->i32{
    x+1
}
- 现在空间里有了一个crate。让adder依赖库crate add-one首先需要在adder/Cargo.toml增建路径：
[dependencies]
add-one = {path = "../add-one"}
- 然后在adder/src/main.rs
use add_one;
- 在add目录中可以直接巡幸cargo build
- 运行的cargo run -p adder
## 在功过空间中依赖外部crate
- 工作空间只有在根目录有一个Cargo.lock而不是在每一个crate目录下都有Cargo.lock，确保相同版本的依赖
如果在cargo.toml和add-one/Cargo,toml增加rand,解析为统一版本并记录到唯一的Cargo.lock中
- 在add-one/Cargo.toml增加[dependencies]部分增加rand crate一边能在crate中使用
- 需要的crate中增加这个dependencies
## cargo test
- 全部测试
- 也可以指定
cargo test -p add-one
## cargo publish
- 需要进入每一个分别的crate进行publish
## cargo install安装二进制文件

# 智能指针
- 指针
    - 是一个包含内存地址的变量的通用概念。地址引用
    - 常见指针就是引用，以&符号为并借用他们所指向的值
- 智能指针
    - 是一类数据结构，他们的表现类似于指针，但拥有额外的元数据和功能
    - 引用计数 reference counting，智能指针类型，其允许数据有多个所有者。
    - string 和 vec。他们属于智能指针，因为他们拥有一些数据并允许你修改他，
    - 实现了Deref和Drop trait， Deref允许他表现得像引用一样，
## Box<T>指向堆上数据
- 最简单的是box，类型是Box<T>，留在栈上的是指针，
- 除了数据被储存到堆上，box没有性能损失，用于：
    - 当有一个在编译时未知大小的类型，而又不想要在需要确定大小的上下文中使用这个类型值的时候
    - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时和，
    - 当希望拥有一个值并只关心它的类型是否实现了特定trait而不是其具体类型的时候
1. box允许创建递归类型部分展示第一种类型，
2. 转移大量数据所有权可能会花费很多哦时间，数据在栈上进行拷贝，少量指针存在
3. trait对象。

## Box<T> 在堆上存储数据
- 创建递归类型
    - 一种无法在编译时知道大小的类型时递归类型
    - 其值的一部分可以时相同类型的另一个值。可以无限下去
- cons list的内容
    - 利用两个参数来构建一个新的列表，通常是一个单独的值和另一个列表
    - cons函数的概念涉及到更常见的函数式变成属于，
    - 当前项的值和下一项，
    - 最后项包含一个nil值并没有狭义相
    - 递归调用cons函数，
    - base case终止条件：规范名称是nil宣布列表的终止。
enum List{
    Cons(i32, List),
    Nil,
}   
    - 递归的：他粗糙南方了另一个相同类型的值，了了解以下rust如何决定需要多少空间来存放一个非递归类型
    - 使用Box<T>给递归类型一个已知的大小
    - 这意味着我们可以将box放入cons成员中而不是直接方另一个list的值
enum{
    Cons(i32, Bos<List>),
    Nil,
} 
    - Cons成员将会需要一个i32大小加上存储box指针数据的空间，Nil成员不存储数据值
    - usize
    - Box<T>类型是一个智慧指针，他实现了Deref trait，他允许Box<T>值被当作引用对待。

## Deref trait 将智能指针当作常规引用处理
- 实现Deref 允许我们重载解引用运算符dereference operator * 与乘法运算符或通配符相区别。
- 通过这种方式实现Deref的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。
- 首先查看解引用运算符如何处理常规引用，
- 解引用强制多态deref coercions功能以及它如何处理引用或智能指针的。
- 构建MyBox<T> 区别：不会在堆上存储数据，

## 通过解引用运算符追踪指针的值
    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);
    assert_eq!(5,y);//错误

- 像引用一样使用Box<T>
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);

- 自定义智能指针
    - Box<T> 被定义为包含一个元素的元结构体
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
    - type Target = T; //定义了用于此trait的关联类型，关联类型是一个稍有不同的定义泛型参数的方式，
    - deref方法体中写入了&self.0，这样deref返回了我希望通过*运算符访问的值引用
    - 没有Deref的话，编译器指挥提供&解引用，deref方法向编译器提供了获取任何实现了Deref trait类型的值，
    - 底层实现了 *(y.deref()),需要 *的原因是所有权问题

## 函数和方法的隐式解引用强制多态
- 解引用强制多态实现了Deref所能够的引用转换为原始类型年通过Deref所能转换的引用，
- 当这种特定类型的引用作为实际参数传递给和形式参数类型不同的函数或者方法时，解引用强制多态将自动发生。
- 这个特定编写时无需增加过多显式使用&和*的引用和解引用，
- 通过调用deref将&MyBox<Sttring> 变成 &String
- &MyBox<String> &(*MyBox<String>)  &String

## 解引用强制多态如何与可变性交互
- Deref不可变引用*
- DerefMut可变
满足以下三种进行解引用强制多态
1. 当T： Deref<Target=U>时从&T - &U
2. 当T： DerefMut<Target=U> &mut T - &mut U
3. 当T： Deref<Target = U> &mut T 到&U
- 第三个情况：Rust也会将可变引用强转为不可变引用，但是反之是不可能的：不可变引用永远也不能强转为可变引用。
- 因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用。
- 将一个可变引用转换为不可变引用永远也不会打破借用规则。

## Drop运行清理代码
- 离开作用域执行代码
- 指定的代码用于释放类似于文件或者网络连接的资源
- 自动插入
- 指定在值离开作用域时应该执行的代码的方式时实现Drop trait,
- drop方法，获取self的可变作用，
- drop trait 并提供了一个调用println!的drop方法实现，drop函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方。
- std::mem::drop提早丢弃值，不能禁用drop这个功能，整个drop的意义在于其自动处理，有时候提早清理某个值
- c.drop()
- 析构函数，对应创建实例的构造函数。
- rust不能显式调用drop，因为rust早在main结尾的时候自动调用drop会有double free错误

## Rc<T>引用计数智能指针
- 启用多所有权，因为大部分情况下所有权是明确的，
- Rc<T> 其名字叫做引用计数reference counting，引用次数，意味着记录一个值引用的数量来知晓这个值是否仍在被使用，
- 如果某个值有0个引用，就代表没有任何有效引用并可以被清理
- Rc<T> 用于当我们希望在堆上分配一些内存给程序的多个部分读取，而无法在编译时确定程序的哪一部份会最后结束使用他的时和，
- 只能用于单线程
### Rc<T>共享数据
- 希望创建两个共享第三个所有权列表，
- b:3->a:5->19
- c:4->a:5
- 共享a所有权
- cons list ： Box
    let a = Cons(5,
        Box::new(Cons(10,
        Box::new(Nil)))
    );

    let b = Cons(3, Box::new(a));
    //Cons成员拥有其存储的数据，所以当创建b列表时，a被移动到了b这样b就拥有a
    //可以改变Cons的定义来存放一个引用，不过接着必须指定生命周期参数。通过指定生命周期参数，表明列表中的每一个元素都至少与列表本身存在的一样久。
    let c = Cons(4,Box::new(a));
- 准备修改List定义为使用Rc<T>替代Box<T>。现在每一个Cons变量都包含一个值和一个指向List的Rc<T>
- 当创建b时，不同于获取a的所有权，这里会克隆a包含的Rc<List>这会将引用计数1增加到2
- 每次调用Rc::clone。数据的引用
enum List{
    Cons(i32, Rc<List>),
    Nil
}
use std::rc::Rc;
let a = Rc::new(Cons(5,Rc::new(10., Rc::new(Nil))));
let b = Cons(3,Rc::clone(&a));//Rc::clone不做深度拷贝，只是增加计数
let c = Cons(4, Rc::clone(&a));


## RefCell<T> 和内部可变形模式
- 内部可变形interior mutability设计模式
- 即使在不可变引用时也可以改变数据，
- 该模式在数据结构中使用unsafe代码来模糊rust可变性和借用规则，
### RefCell<T> 在运行时检查借用规则
- RefCell<T>代表其数据的唯一的所有权，
- 借用规则：
    - 在任意时刻，只能拥有一个可变引用或者任意数量的不可变引用之一（而不是两者）
    - 引用必须总是有效的
- Box<T>借用规则的不可变性作用于编译时，
- RefCell<T>这些不可变形作用于运行时，panic，当你确定代码遵守和借用规则，但是编译器不能理解和确定的时候
- Rc<T>允许相同数据有多个所有者；box和refcell有单一所有者
- box允许在编译时执行不可变或可变借用检查，Rc仅允许在编译时执行不可变借用检查，refcell允许在运行时执行不可变或者可变借用检查
- 因为refcell允许在运行时执行可变借用检查，所以我们可以在即便RefCell自身不可变的情况下修改其呢逆不知
let x = 5;
let y = &mut x;
// 会有编译错误，推论当有一个不可变的值，不能可变地
- 然而在特定情况下，令一个值在其方法内部就能修改自身，而在其他代码中仍然视为不可变，是有用的
- mock对象
    - 测试替身test double是一个通用的编程概念，他代表一个在测试中代替某一个类型的类型，mock对象是特定类型的测试替身，
    - 记录与最大值的差距。

- 在运行时记录借用
    - 当创建不可变和可变引用时，分别使用&和&mut语法，对于RefCell<T>来说，则是borrow和borrow_mut
    - 返回RefMut类型的只能指针。都实现了Deref所以可以当作常规引用对待
    - RefCell<T>记录当前有多少个活动的Ref<T>和RefMut<T>安全API的一部分。
    - borrow返回的时Ref<T>类型的智能指针，borrow_mut方法返回RefMut类型的智能指针，这两个类型都是县了Deref。所以都可以当作常规引用对待
    - 每次调用borrow, RefCell将活动的不可变借用计数加一，当Ref值李离开作用域时，不可变借用计数减1。
    - RefCell在任何时和只允许多个不可变借用或者一个可变借用
    - RefCell的实现会在运行时出现panic
    - 在运行时捕获借用错误而不是编译时意味着将会在开发过程的后期才会发现错误，甚至有可能发布到生产环境才发现，

### 结合Rc和RefCell来拥有多个可变数u所有者
- Rc允许对相同数据有多个所有者，不过只能提供数据的不可变访问。
- 如果有一个存储了RefCell和Rc的话，就可以得到有多个所有者并且可以修改的值
- 创建一个Rc<RefCell<i32>>实例并存储存在变量value中以便之后直接访问，
- 接着在a中包含value的cons成员创建了一个list,需要克隆value 以便a和value都能拥有其内部5的所有权，而不是将所有权从value 转移到a或者让a借用value.
- value调用了borrow_mut返回智能指针，可以对其使用解引用运算符并修改其内部值
- 表面上不可变的list，不过可以使用RefCell中提供内部可变的方法在需要时修改数据，免于出现数据竞争。
- 比如Cell类似于RefCell但有一点除外：它并非提供内部值的引用。而是将值拷贝进和拷贝出Cell。还有Mutex。其提供线程间安全的内部可变性

### 引用循环与内存泄漏
- Rust的内存安全性保证使其难以意外地制造永远不会被清理的内存，内存泄漏memory leak
- 与在编译时拒绝数据竞争不同，并不保证完全地避免内存泄漏。创建引用循环的可能性是存在的。
- 制造引用循环
    - 一个存放RefCell的cons list定义，这样可以修改Cons成员所引用的数据
    - List定义的另一种变体，RefCell<Rc<List>>,这意味着不能像上面那样修改i32的值，我们希望能够修改Cons成员指向的List。这里还增加一个trait方法来方便我们来访问
     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item {:?}", b.tail());

    if let Some(link) = a.tail(){
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    - 创建一个引用循环，两个List值互相指向彼此
    - 变量a中创建一个Rc<List>实例来存放初值5，Nil的list值。解州b存放10和指向列表a的list的另一个Rc
    - 创建引用循环之后程序立刻结束
- 避免引用循环:将Rc<T>变成Weak<T>
    - 也可以通过调用Rc::downgrade并传递Rc<T>实例的引用来创建其值的弱引用weak reference
    - Rc::downgrade会得到Weak<T>类型的智能指针，不同于将Rc<T>实例的strong_count加1，Rc::downgrade将weak_count加1
    - 强引用代表如何共享Rc<T>实例的所有权，但若引用并不属于所有权关系，不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为0被打断。
    - 因为Weak<T>引用的值可能已经被丢弃，为了使用weak<T>所指向的值，我们必须确保其值仍然有效。为此可以调用weak<T>实例的upgrade方法，这会返回Option<RC<T>>
    - 如果Rc<T>值还未被丢弃，返回some

## 创建树形数据结构：带有子节点Node
- node 拥有其子节点，同时西塘通过变量共享所有权，Rc<Node>, Vec<T>
    - 修改其他节点的子节点 Vec<Rc<Node>> 放入RefCel<T>
- 增加子到父的引用
    - 为了让子节点知道父节点增加parent字段，
    - parent类型？不要引用循环
        - 父节点应该拥有其子节点，
        - 如果父节点被丢弃了，其子节点也应该被丢弃，然而这个子节点不应该包含父节点
        - 如果子节点丢弃，父节点仍然存在，这就是弱引用
        - RefCell<Weak<Node>>