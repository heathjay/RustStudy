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

# 无畏并发
- 并发编程：程序的不同部分相互独立执行，concurrent programming
- 并行编程：程序不同部分同时执行，parallel programming
- 所有权和类型系统是解决内存安全和并发问题的工具fearless concurrency
包括
    - 如何创建线程来同时运行多段代码
    - 消息传递message passing， 其中通道channel被用来传递消息
    - 共享状态shared state,其中多个线程可以访问同一片数据
    - Sync和Send trait， 将并发保证扩展到用户定义的以及标准库提供的类型中

## 使用线程同时运行代码
- 已执行程序的代码在一个进程中process，程序内部可以拥有多个同时运行的独立部分，运行这些独立部分的功能被成为线程
- 问题：
    - race condition，多个线程以不一致的顺序访问数据或者资源
    - deadline：两个线程互相等待对象停止使用其所拥有的数据或者资源，这会组织他们继续运行
    - 指挥发生在特定情况且难以稳定重现和修复的bug
- 编程语言提供的线程被称为绿色green线程，使用绿色线程的语言会在不同数量的os线程的上下文执行他们，称为M:N模型，
- Rust是较为底层语言，只提供1：1
## 使用spawn创建新线程
- thread::spawn函数并传递一个闭包，并在其中包含希望在新线程运行的代码，

## 使用join等待所有线程结束
- 可以通过thread::spawn的返回值存储在变两种来修复新建线程部分没有执行或者完全没有执行的问题
- thread::spawn的返回值类型是JoinHandle,是一个拥有所有权的值，当对其调用join方法的时候，他会等待线程结束
- 阻塞当前线程，知道handle代表的线程结束
- join放在不同的地方会影响线程
## 线程和move闭包
- 允许我们在一个线程中使用另一个线程的数据
- 创建爱你新线程将值的所有权从一个线程转移到另一个ie线程
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    - 闭包没有参数传入
- rust不知道这个线程会执行多久，无法知道引用是否一直有效
- 在闭包之前增加move关键字，我们强制闭包获取其使用的值的所有权，而不是任由rust推断他应该借用值

## 使用消息传递在线程间传送数据
- 不要通过共享内存来通讯，通过通讯来共享内存
- channel
- 一个发送者transmitter和一个接受者receiver
- 任意一个被丢弃了就可以认为通道被关闭
    let (tx, rx) = mpsc::channel();
- 多个发送一个接受
- send方法返回一个result类型，将没有发送值的目标所以发送操作会返回错误unwrap产生panic
- 接收端有两个方法，一个recv和try_recv。这个会阻塞朱线程执行直到从通道接受一个值，一旦发送了一个值，recv会一个result中返回。当通道关闭就返回一个错误
- try_recv不会阻塞，立刻返回一个result:ok包含可用的信息，err代表没有任何信息

## 通道与所有权转移
- 尝试在新建线程中的通道中发送完val值之后再使用它，不可以
- send 函数获取其参数的所有权并移动这个值归接受者所有，防止发送后再次意外地使用这个值，所有权系统检查一切是否合乎规则

## 发送多个值并观察接受者的等待
- 新建线程现在会发送多个消息并在每个消息之间暂停一秒钟
thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            
            ];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx{
        println!("got : {}", received);
    }
- 不再显式调用recv函数：而是将rx当作一个迭代器，对于每一个接收到的值我们将他打印出来，当通道被关闭时，每一行都会暂停一秒


## 通过克隆发送者来创建多个生产者
- mspc:multiple producer, single consumer
  let tx1 = mpsc::Sender::clone(&tx);
- 一个可以传递给第一个新建线程的发送端句柄，我们将原始的通道发送端传递给第二个新建线程。
## 共享状态并发
- 消息传递是一个很好的处理并发的方式
- 但不是唯一一个
- 某种程度上，通道类似于单所有权，一旦将一个值传送到通道中，将无法再使用这个值
- 共享内存类似于多个所有权，多线程可以同时访问相同的内存位置，
### 互斥一个线程访问数据
- mutex
- 线程首先需要通过获取互斥的锁来表明其希望访问数据，锁是一个作为互斥一部分数据结构
- new创建一个mutex，使用lock方法锁
- lock返回一个MutexGuard的智能指针
- 实现了Deref指向内部数据，提供了一个Drop实现当MutexGuard离开作用嗯与自动释放

### 线程间共享Mutex
- 多线程和多所有权
- 尝试使用智能指针Rc<T>来创建计数的值，以便拥有多个所有者。
- Rc不能安全的在线程间共享，并没有clone
- 类似与Rc，但具有原子性Arc<T>
atomic/ atomically reference counterd

    let counter = Arc::new(Mutex::new(0));
    let counter = Arc::clone(&counter);
                let mut num = counter.lock().unwrap();
                *num += 1;
     println!("result:{}", *counter.lock().unwrap());
- counter 不可变，但是可以获取他内部值的可变引用，就像cell一样
- 可能会造成死锁


### 使用sync和Send trait的可扩展并发
- 以上都是标准库api，
- 我们可以自己编写自己的或者使用别人编写的并发功能
- std::marker sync和send trait
- 通过send可以实现所有权的转移
- 几乎所有rust类型都是send trait有些例外：Rc
- Sync 允许多线程访问
    - 对于任意类型T，如果&T时send的话，T就是sync的，意味着他引用就可以安全的发送给另一个程序
    - Rc和RecCell不是sync
    - 手动实现send和sync时不安全的，
- 通常需要crate，




# Rust的面向对象特性

## 面向对象语言特性
- 对象包含数据和行为
- 结构体和枚举包含数据而impl提供了结构体和枚举之上的方法。
### 封装隐藏了实现细节
- 对象编程相关，对象的实现细节不能被使用对象的代码获取，唯一与对象交互的方式是通过对象提供的公有api。
- 默认私有化，pub可以
### 继承作为类型系统与代码共享
- inheritance：一个对象可以定义为继承另一个对象的定义使其可以获得父结构的数据和行为，而无需重新定义
- rust没有，但提供了解决方法
    - 选择继承有两个主要的原因：重用代码，可以用trait
    - 子类对象可以用于父类被使用的地方，被称为多态polymorphism；可以进行泛型编程

## 为使用不同类型的值而设计的trait对象
- 克服ector只能存储同种类型号元素的局限，
- 可以提供一个定义SpreadsheetCell枚举来存储整形，浮点型和文本成员的替代方案。可以在单元中存储不同类型的数据，
- 设计一个gui
### 定义通用行为的trait
- 定义一个Draw trait，其中包含名为draw的方法
- 接着定义一个存放trait对象的vector。trait对象值西那个一个实现了我们指定trait的类型的实例，以及一个用于在运行时查找该类型的trait方法的表
- 可以使用trait对象替代泛型或具体类型，任何使用trait对象的位置
- 不能向trait对象增加数据
- trait对象则允许在运行时替代多种具体类型，泛型智能一次智能替代一个具体类型
pub struct Screen{
    pub components:Vec<Box<dyn Draw>>,
}
impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

/**
 * 泛型和trait bound
 */
/*
pub struct Screen<T: Draw>{
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T:Draw{
        pub fn run(&self){
            for component in self.components.iter(){
                component.draw();
            }
        }
    }
*/
- 倾向于使用trait对象，一个Screen实例可以存放一个既能包含Box<Button>也能包含Box<TextField>
### 实现trait
- 增加类型
- 实现trait和方法
- 只关心值所反映的信息而不是具体类型-类似于动态类型语言中的鸭子类型duck typing：如果走起来像一只鸭子，叫起来也是就是
- run不需要知道每个组件的具体类型是什么，通过Box<dyn Draw>作为component vector中的值，定义了screen为需要可以在其上调用draw方法的值

### trait对象执行动态分发
- 对于泛型使用trait bound 时编译器进行单态化处理：
    - 编译器为每一个被泛型类型参数替代的具体类型生成非泛型的函数和方法实现
    - 单态化产生的代码进行静态分发 static dispatch:编译的时候就知道了
    - 动态分发dynamic dispatch相对
    - 使用Trait对象，必须动态分发，编译器无法知道所有可能用于trait对象代码的类型。所以不知道调用哪个类型的那个方法实现，
    - 通过调用指针知道
### trait对象要求对象安全
- object safe的trait才可以组成trait对象，
- 两条规则：
    - 返回值类型不为self
    - 方法没有任何泛型类型参数
    - trait对象忘记了其真正的类型，方法不能使用已经被忘记的self类型
    - 同理对于泛型类型的参数，当trait时放入具体的类型参数：此具体类型编程实现该trait类型的一部分当时用trait对象时其具体类型被抹去了
    - 不安全的例子：clone trait
    pub trait Clone{
        fn clone(&self) -> Self;
    } 

# 面向对象设计模式的实现
- 状态模式state pattern。关键在于一个值有某些内部状态，体现为一些列的状态对象，同时值的行为随着其内部状态而改变
- 状态对象共享功能，每一个状态对象负责其自身行为。以及该状态如何转移另一个状态。持有一个状态对象的值对于不同状态的行为以及何时状态转移毫不知情

## 发布博客的实现
- 博文从空白开始
- 草案完成。请求审核博文
- 一旦博文过审，发表
- 只有发表的博文会被打印，这样就不会意外打印没有被审核的博文的文本

pub struct Post{
    state: Option<Box<dyn State>>,
    content:String,
}
impl Post{
    pub fn new()->Post{
        state: Some(Box::new(Draft{})),
        content:String::new(),
    }
}
//定义了不同状态的博文所共享的行为
trait State{
}
struct Draft{}

impl State for Draft{}
- add_text
- 确保博文草案时空的
-  fn request_review(self:Box<Self>) -> Box<dyn State>;
    - self:Box<Self> 意味着这个方法调用只对这个类型的Box有效，获取了Box<Self>的所有权，使得老状态无效
- 增加改变content行为的approve方法
- 状态模式的权衡取舍
    - 要找到所有已发布博文的不同行为只需要查看published的state trait实现
    - 如果创建一个不是用状态模式的替代实现。可能在post的方法中甚至于main中甬道match，需要看更多位置才能理解发布的博文的所有逻辑
- 缺点：
    - 状态间实现了状态转换，相互联系，
    - 重复逻辑，
    - post
- 进行改进：
 
# 模式用来匹配值的结构
- 模式是rust中特殊的语法，他用来匹配类型的结构。
- 结合使用模式和match表达式以及其他结构可以提供更多对程序控制流的支配权。
- 模式：
    - 字面值
    - 解构的数组、枚举、结构体或者元组
    - 变量
    - 通配符
    - 占位符
- 这些描述了我们需要处理的数据形状接着可以用其匹配值来决定程序是否拥有正确的数据来运行特定部分的代码
## 所有可能用到模式的地方
- match分支
match value{
    Pattern => expression,

}
    - 必须穷尽，所有值都必须被考虑到，

## if let 条件表达式
- 等同于只关心一个情况match语句简写的。
## while let 条件循环
- 允许只要模式匹配就一直进行while循环
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);
//一旦none就是退出
while let Some(top) = stack.pop(){
    println!("top is {}", top);
}
## for 循环
- 模式是for关键字直接跟随的值
- 如for x in y中的x
let v = vec!['a','b','c'];
for (index, value) in v.iter().enumerate(){
    println!("{} is at index.", value, index);
}

## let 语句
let x = 5;
- 正式的是:
let Pattern = Expression;
- x是整个模式。这个模式实际上将任何值绑定到变量x，不管值是什么
let (x, y ,z ) = (1, 2, 3);
- 比较值1,2,3和x,y,z并发现此值匹配这个模式
- 如果希望忽略元组中一个或多个值，也可以使用_或这..,如忽略模式中的值部分所示，如果问题是模式中有太多的变量，则解决方法是通过去掉变量使得变量数与元组中元素数相等

## 函数参数
- 也可以是模式
fn foo(x:i32){}
- x部分就是模式
fn print_coordinates(&(x,y) : &(i32, i32)) {
    println!("current location:{} , {}", x, y);
}
fn main(){
    let point = (3,5);
    print_coordinates(&point);
}
- 值&(3,5)匹配模式&(x,y)
- 闭包类似于函数，也可以在闭包参数列表中使用模式，
- 不过模式在每个使用它的地方并不以相同的方式工作，在一些地方，模式必须是irrefutable的，意味着他们必须匹配所提供的任何值
- 在另一些情况。他们则可以使用refutable的。

## refutability 可反驳性，模式是否会匹配失效
- 对某些可能的值进行匹配会失败的模式被称为是可反驳的refutable
if let Some(x) = a_value{...}
- if let和while let 表达式被限制为只能接受可反驳的模式，因为根据定义他们在处理可能的失败
- 熟悉可反驳的概念
## irrefutable 不可反驳性
- 能匹配任何传递的可能值的模式被称为是不可反驳的irrefutable
- let x = 5; //x可以匹配任何值所以不可能失败
- 函数参数，let语句和for循环只能接受不可反驳的模式，因为通过不匹配的值程序无法进行有意义的工作，
- 为了修复在需要不可反驳模式的地方使用可反驳模式的情况，可以修改使用模式的代码：
    - 不同于使用let，可以使用if let。如果模式不匹配，大括号中的代码将被忽略，其余代码保持有效
## 所有模式语法
### 匹配字面值
let x = 1;
match x{
    1 => println!("one"),
    2 => println!("two");
    _ => println!("anything");
}
### 匹配命名变量
- 命令变量是匹配任何值的不可反驳模式
- match会开始一个新的作用域。match表达式作为模式的一部分生命的变量会覆盖match结构之外的同名变量
    let x = Some(5);
    let y = 10;

    match x{
        Some(50) => println!("got 50"),
        Some(y) => println!("matched, y = {:?}",y),
        _ => println!("default case x ={:?}",x),
    }
    - match第二个匹配分支引入新的变量y，他会匹配Some中的任何值，结束后作用域结束，
- 多个模式
    - 在match表达式中，可以使用|语法匹配多个模式，它代表或or
        let x = 1;
    match x{
        1|2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
- 通过..=匹配值的范围
..=允许匹配一个闭区间范围内的值，
    match x{
        1..=5 =>println!("one through five"),
        _=>println!("something else"),
    }
    // x 是1，2，3，4，5第一个分支就会匹配
    - 只允许数字和char类型
    match x{

    }
## 解构并分解值
- 可以使用模式来结构结构体、枚举、元组和引用
    //解构结构体
    let p = Point{x:0,y:8};
    let Point{x:a,y:b} = p;
    assert_eq!(0,a);
    assert_eq!(8,b);
    //变两名匹配字段名
    let Point{x,y}= p;
    //简写，写出字段名称。模式创建的变量会有相同的名称
    assert_eq!(0,x);
- 允许部分解构
    - 分成三部分：直接位于x轴上，此y=0真，位于y轴，或者不再任何轴点上
        match p{
        Point{x, y : 0} => println!("on the x axis at {}",x),
        Point{x : 0,y} => println!("on the y axis at {}", y),
        Point{x,y} => println!("on neither axis:{} {}", x, y),
    }
- 结构枚举
enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
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

- 解构嵌套的结构体和枚举
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
    let msg = Message::ChangeColor(Color::Hsv(0,160,255));
    match msg{
        Message::ChangeColor(Color::Hsv(h,s,v))=>println!("hsv:{},{},{}",h,s,v),
        Message::ChangeColor(Color::Rgb(r,g,b)) => println!("rgb:{},{},{}",r,g,b),
        _ => ()
    }

- 解构结构体和元组
一个复杂的解构体例子，其中结构体和元组嵌套在元组中，并将所有的原始类型解构出来：
let ((feet, inches), Point{x,y}) = ((3,10), Point{x:10,y:-19});
- 模式解构可以部分
- 忽略模式中的值
    _, 使用一个以下划线开始的名称，
    ..忽略所剩部分的值，
- 用_忽略整个值
fn foo(_:i32, y:i32){
    println!("This code only uses the y parameter: {}", y);
}
- 使用嵌套_忽略部分值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }
- 通过在名字前以一个下划线开头来忽略未使用的变量
    - 不要警告未使用的变量，为此可以使用下划线作为变量的开头
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

- 用..忽略剩余值
    - 对于有多个部分的值，可以使用..语法来只使用部分并忽略其他值，同时避免不得不每一个忽略值列出下划线
    struct Point{
        x:i32,
        y:i32,
        z:i32
    }

    let origin = Point{x:0, y:0, z:0};
    match origin{
        Point{x,..} =>println!("x is {}",x);
    }

    - 或者
    let numbers = (2, 4, 8, 16, 32);
    match numbers{
        (first,..,last) => println!("some numbers:{}, {}",first, last);
    }
    - 不能有歧义，(.., second, ..); 会错

- 匹配守卫提供的额外条件
    - 匹配守卫: match guard指定于match分支模式之后的额外if条件，它必须被满足才能选择此分支，
    - 匹配守护用于表达比单独的模式所能允许的更为复杂的情况
        let num = Some(4);
    match num{
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}",x),
        None => ()
    }
- @ 绑定
    - 允许我们创建一个存放值的变量的同时测试其值是否匹配模式


# 高级特征
- 不常见但很有用
## 不安全Rust
- 编译时都会强制执行的内存安全保证，
- 隐藏第二种语言，它不会强制执行这类内存安全保证：不安全rust
- unsafe rust
- 如果rust强制安全，有部分操作无法进行
- 底层计算机硬件故有的不安全性，如果rust不允许进行不安全操作，那么有些任务根本完成不了，
### 不安全的超级力量
- 通过unsafe关键字来切换成不安全Rust，可以开启一个新的存放不安全代码块
- 5类可以在不安全的rust中执行而不能在安全的rust操作:
    - 解引用裸操作
    - 调用不安全的函数或方法
    - 访问或修改可变静态变量
    - 实现不安全trait
    - 访问union的字段
- 并不会关闭其他rust安全检查：如果在不安全代码中使用引用。它仍然会被检查。unsafe关键字值是提供了那五个不会被编译器就爱你茶内存安全的功能。
- 作为程序员要确认unsafe代码以有效的方式访问内存
- 保持unsafe足够小

### 解引用裸指针
- 不安全rust有两个被称为裸指针raw pointer的类似于引用的新类型，和引用一样，裸指针是可变或者不可变的，
- *const T 和 *mut T.这里的星号不是解引用运算符
- 它是类型名称的一部分，在裸指针的上下文中，不可变意味着指针解引用之后不能直接赋值
- 与引用和智能指针的区别在于，裸指针：
    - 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
    - 不保证指向有效的内存
    - 允许为空
    - 不能实现任何自动清理功能
- 通过去掉Rust强加的保证，可以放弃安全保证以换取性能或使用另一个语言或硬件接口的能力，此时rust的保证并不适用
- 创建如何从引用同时创建不可以便和可变裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe{
        println!("r1 is: {}", *r1);
        println!("r2 is : {}", *r2);
    }
- 通过引用创建裸指针
- 可以在安全代码中创建裸指针，值是不能在不安全块之外解引用裸指针
- 这里使用as将不可变和可变引用强转为对应的裸指针，因为直接从保证安全的引用来创建他们，可以知道这些特定的裸指针
- 展示如何创建一个指向任意内存地址的裸指针，尝试使用任意内存是未定义行为：此地址可能有数u也可能没有，编译器可以优化掉这个内存访问，或者程序可能会出现段错误segmentation fault
    let address = 0x012345usize;
    let r = address as *const i32;
- 创建一个指针不会造成任何危险，只有访问其指向的值才可能遇到无效的值。
- 通过裸指针，就能同时创建同一地址的可变指针和不可变指针，若通过可变指针修改数据，则可能潜在造成数据竞争。
- 主要应用场景就是调用c代码接口，另一个场景就是构建借用检查其无法理解的安全抽象的例子

### 调用不安全函数或方法
- 使用不安全块的操作是调用不安全函数。不安全函数和方法与常规函数方法十分类似。除了其开头有一个额外的unsafe
- 不安全函数体也是有效的unsafe块，所以在不安全函数中进行另一个不安全操作时无需增加额外的unsafe块
### 创建不安全代码的安全抽象
- 仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的。
- 将不安全代码封装进安全函数是一个常见的抽象。
- 一个安全函数定义于可变slice值上，他获取一个slice并从给定的索引参数开始将其分为两个slice。
- 将split_at_mut实现为函数而不是方法，并只处理i32值而非泛型T的slice
fn split_at_mut(slice: &mut [i32], mid:usize) -> (&mut [i32], &mut [i32]){
    let len = slice.len();
    assert!(mid<= len);
    (&mut slice[..mid], &mut slice[mid..])
}
- rust的借用检查器不能理解我们要借用这个slice的两个部分，它只知道我们借用了同一个slice两次，本质上借用slice的不同部分时可以的，因为不重叠
use std::slice;
fn split_at_mut(slice:&mut [i32], mid:usize) ->(&mut [i32], &mut [i32]){
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe{
        (slice::from_raw_parts_mut(ptr,mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}
- as_mut_ptr方法访问slice的裸指针，返回一个*mut i32
- 获取一个裸指针和一个长度来创建一个slice.
- add放入unsafe块中，确认地址偏移


### 使用extern 函数调用外部代码
- 有助于创建和使用外部函数接口foreign function interface.
- 允许不同或外部编程语言调用函数
- extern块中声明的函数在rust代码中总是不安全的。因为其他语言不会强制执行rust规则并且rust无法检查他们所以确保其安全是
- 在extern "C"块中，列出了我们希望能够调用的另一个语言中的外部函数的签名和名称。
- "C" 部分定义了外部函数所使用的应用程序接口application binary interface，定义了如何在汇编语言层面调用此函数。
- "C" ABI是最常见的，
### 从其他语言调用rust函数
- 不同于extern块，就在fn关键字之前增加extern关键字并指定所用到的ABI，还需要增加#[no_mangle]追俄，
- mangling发生于当编译器将我们指定的函数名修改为不同的名称时，这会增加用于其他编译过程的额外信息，不过会使名称更加难以阅读
- 一旦其编译为动态库并从c语言中连接，call_from_c函数就在能够在c代码中访问：
#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("just called a rust function from c!");
}

### 访问或修改可变静态变量
- 全局变量对于rust是一个问题，不过所有权规则说是有问题的。如果两个线程访问相同的可变全局变量，则可能会造成数据竞争。
- 全局变量在rust称为静态变量static，
- 静态变量必须标注变量的类型
static HELLO_WORLD : &str = "hello, world";
- 只能用'static生命周期，固定内存地址，使用这个值总会访问相同的地址另一方面，常量则允许在任何被用到的时候复制其数据。
- 访问和修改可变静态变量都是不安全的。
static mut COUNTER:u32 = 0;
fn add_to_count(inc:u32){
    unsafe{
        COUNTER += inc;
    }
}
    add_to_count(3);
    unsafe{
        println!("{}",COUNTER);
    }
- 加上mut
- unsafe块加入
- 多线程容易数据竞争，

### 实现不安全的trait
- 当至少有一个方法中包含编译器不能验证的不可变时，trait是不安全的，可以在trait之前增加unsafe关键字将trait声明为unsafe
unsafe trait Foo{
    //methods go here
}
unsafe impl Foo for i32{

}
- Sync和Send标记trait，编译器会自动为完全由Send和Sync类型组成的类型自动实现他们。
- 如果实现了一个包含了一些不是Send或Sync的类型，并希望将此类标记为Send或Sync，则必须使用unsafe。

### 访问联合体中的字段
- union和struct类似，
- 一个实例中同时只能使用使用一个声明的字段，
- 联合体主要用于和C代码的联合体交互，访问联合体的字段是不安全的，因为rust无法保证当前存储在联合体实例中数据的类型
### 何时使用不安全代码
- 使用unsafe来进行这五个操作之一是没问题的，
- 不过使得unsafe代码正确也实属不易
- 因为编译器不能帮助保证内存安全。
- 当有理由使用unsafe代码时，是可以这么做的，通过使用显式的unsafe标注使得在出现错误时易于追踪问题的源头。

## 高级trait
### 关联类型在trait定义中指定占位符类型
- associated types-关联类型，是一个将类型占位符和trait相关联的方式，这样trait的方法签名中就可以使用这些占位符类型了
- 带有关联类型来替代遍历的值的类型，
pub trait Iterator{
    type item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>
}
- Item就是一个占位类型，同时next方法定义表明他返回Option<Self::Item>类型的值，
- 这个trait的实现者指定了Item类型，然而不管实现者指定何种类型，next方法都会返回一个包含了此具体类型值的Option
- 关联类型看起来像一个泛型的概念，它允许定一个函数而不指定可以处理的俄类型
- 为什么不进行泛型?
pub trait Iterator<T>{
    fn next(&mut self) -> Option<T>{}
}
- 如果使用泛型的话，则不得不在每一个实现中标注类型，这是因为我们也可以实现Iterator<String> for counter,
- 或其他类型，当trait具有泛型参数的时候，可以多次实现这个trait，每次需要改变泛型参数的具体类型，
- 接着如果使用Counter的next方法时，必须提供类型注解来表明希望使用Iterator的哪一个实现。
- 通过关联类型，则无需标注类型因为不能多次实现这个trait。我们只能使用选择一次Item会是什么类型，因为只能有一个impl Iterator for Counter
- 当调用Counter的next时不必每次指定我们需要u32值的迭代器

### 默认泛型类型参数和运算符重载
- 当使用泛型类型参数时，可以为泛型指定一个默认的具体类型，如果默认类型就足够的话，这消除了为具体类型实现trait的需求
- 运算符重载operator overloading- 并不允许但是std::ops中所列出的运算符和相应的trait可以通过实现运算符相关trait来重载
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

fn main() {
    assert_eq!(Point{x : 1, y : 0} + Point{x:2, y:3}, Point{x:3,y:3});
}

- 这里默认泛型类型位于Add trait中，这里定义：
trait Add<RHS=Self>{
    type Output;
    fn add(self, rhs:RHS) -> Self::Output;
}
- 这是一个带有一个方法和一个关联类型的trait
- 尖括号RHS=Self这个语法叫默认类型参数default type parameter
- RHS 泛型类型参数right hand side 缩写，它用于定义add方法中rhs参数
- 如果实现add trait时不指定RHS的具体类型，RHS的类型将时默认的Self类型，也就是在其上实现add的类型。
- 使用自定义类型
use std::ops::Add;
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters{
    type Output = Millimeters;
    fn add(self, other:Meters) -> Millimeters{
        Millimeters(self.0 + (other.0 * 1000))
    }
}
- 默认参数类型主要用于如下两个方面：
    - 扩展类型而不破坏现有代码
    - 在大部分用户都不需要的特定情况进行自定义

### 完全限定语法与消歧义：调用相同名称的方法
- rust既不能避免一个trait与另一个trait拥有相同名称的方法，也不能阻止为同一类型同时实现这两个trait，甚至直接在类型上实现开始已经有的同名方法也是可能的
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
    
    
    //直接调用
    //直接调用实现在human的fly方法
    let person = Human;
    person.fly();

    //用更明显的方法调用。
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

- 因为fly方法获取一个self参数，如果有两个类型都实现了同一个trait，Rust可以根据self的类型计算出应该使用哪一个trait实现
- 关联函数是trait的一部分，但没有self参数，当同一作用域的两个类型实现了同一个trait，Rust就不能计算出我们期望哪一个
- 除非使用完全限定语法fully qualified syntax。
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


    println!("a bady dog is called a {}", Dog::baby_name());
    /*
    rust并不知道该使用哪一个实现
    因为是关联函数而不是方法，没有self参数，会有编译错误
    所以使用完全限定语法
    println!("a bady dog is called a {}", Animal::baby_name());
    */
    println!("a bady dog is called a {}", <Dog as Animal>::baby_name());

- 尖括号中向rust提供了类型注解，并通过在此函数调用中将Dog类型当作Animal对待，来指定希望调用的是Dog上Animal trait
<Type as Trait>::function()

### 父trait用于在另一个trait中使用某trait的功能
- supertrait
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
//必须实现这个才能让OutlinePrint应用到pointer上
impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({},{})",self.x,self.y)
    }
}
impl OutlinePrint for Point{}

### newtype模式用以在外部类型上实现外部trait
- 孤儿规则orphan rule，它说明只要trait或者类型对于当前crate是本地的话就可以实现该trait，一个绕开这个限制的方法是使用newtype模式。它涉及到一个元组解构题中创建一个新类型
- 这个元组解构题带有一个字段作为希望实现trait类型的简单封装，接着这个封装类型对于crate是本地的，这样就可以在这个封装上是实现trait
- 使用这个模式没有运行时性能惩罚
- 如果想在vec上实现display，而孤儿规则组织我们这样直接作，因为display trait和vec都定义于我们的crate之外。可以创建一个包含vec实例的wrapper解构体

//newtype
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f, "[{}]",self.0.join(", "))
    }
}
   
   
    //newtype
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

- Display的实现使用self.0来访问其内部的Vec<T>,因为Wrapper是元组结构体而Vec<T>是结构体宗位于索引0的项，
- 因为Wrapper中Display的功能了
- 缺点是Wrapper是一个新类型，没有定义于其值之上的方法；必须直接在Wrapper上实现所有方法

## 高级类型
- newtype模式可以用于静态的确保其值不被混淆
- 和用来表示一个值的单元，
- 另一个newtype模式的应用在于抽象了一些类型的实现细节
- 隐藏内部泛型

### 类型别名用来创建类型同义词
- 类型别名type alias的能力，使用type关键字来给予现有类型另一个名字。
type kilometers = i32;
- 减少重复
type Result<T> = std::result::Result<T, std::io::Error>;
- 易于编写并在整个std::io中提供了一致的接口

### 从不返回never type
- rust有一个叫做!的特殊类型，empty type，因为它没值，倾向于称为never type.
- 在函数从不返回的时候当返回值
fn bar() -> !{

}
- 从不返回，发送函数
let guess: u32 = match guess.trim().parse(){
    Ok(num) => num,
    Err(_) => continue,
}
- continue - !
- 描述!的行为的正式方式是never type可以强转为任何其他类型，允许match的分支以continue结束因为continue并不真正返回一个值；相反它把控制权交回上层循环
- nevertype 另一个用途是panic！
impl<T> Option<T>{
    pub fn unwrap(self) -> T{
        match self{
            Some(v)=>v,
            None => panic!("called option::unwrap()"),
        }
    }
}

### 动态大小类型和sized trait
- 需要知道例如应该为特定类型的值分配多少空间这样的信息其类型系统的一个特定的角落可能令人迷惑：这就是动态大小类型，dynamically sized types.
- unsized types,这些类型允许我们处理只有在运行时才知道大小的类型
- str
- 只有运行时我们都不知道字符串有多长，因为直到运行时都不能知道大小，也就意味着不能创建str类型的变量，也不能获取str类型的参数
- let s1: str= "hello there!"//这个不能工作
- 虽然&T是一个存储了T所在的内存位置的单个值，&str是两个值，长度和地址，在编译时间就知道大小usize两倍。
- 常规用法：他们有一些额外的元信息来存储动态信息的大小，
- 必须将动态大小类型的值在某种指针之后
- 可以将str和所有类型的指针结合:比如Box<str>或者Rc<str>
- 为了将trait用于trait对象，必须将他们放在值镇之后比如 &dyn Trait 或者Box<dyn Trait>
- 有一个特定的trait来决定一个类型的大小是否在编译时间时可知：sized trait，这个trait自动为编译器在编译时就知道大小的类型实现，
- 另外rust隐式地为每一个泛型函数增加了sized bound
fn generic<T>(t:T){

}
fn generic<T:Sized>(t:T){}
- 泛型函数默认只能用于在编译时已知大小的类型，然而可以使用如下方法来放宽这个限制
fn generic<T:?Sized>(t：&T)
- ?Sized trait bound与sized相对，也就是说，它可以读作T可能也可能不是Sized

### 高级函数和闭包
### 函数指针
- 也可以向函数传递常规函数，希望传递已经定义的函数而不是重新定义闭包作为参数是很有用的
- 通过函数指针可以将函数作为参数给了另一个函数
- fn被称为函数指针function pointer
- 不同于闭包，fn是一个类型而不是一个trait，所以直接指定fn函数而不是声明一个带有Fn作为trait bound的泛型参数
- 函数指针实现了所有三个闭包trait(Fn, FnMut和FnOnce)
- 所以总可以在调用期望闭包的函数时传递函数指针作为参数，参数于编写使用泛型和闭包trait的函数，这样他就能接受函数或闭包作为参数
- 一个只期待接受fn而不接受闭包的情况的例子与不存在闭包的外部代码交互时：C语言的函数可以接受函数作为参数，但c语言没有闭包
- 作为一个既能使用内联定义的闭包又可以使用命名函数的例子，
let list_of_numbers = vec![1,2,3];
let list_of_strings = Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
- 或者可以将函数作为map的参数来代替闭包
let list_of_numbers = vec![1,2,3];
let list_of_strings = Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
- 这里必须使用高级trait部分讲到的完全限定语法，因为存在多个叫做to_string函数，
- 另一个实用的模式暴露了元组结构体和元组结构枚举成员的实现细节，这些项使用()作为初始化语法，同时返回i由参数构造的实例的函数，实现了闭包trait的函数指针，
enum Status{
    Value(i32)，
    Stop,
}
let list_of_statuses: Vec<Status>{
    (0u32..20)
.map(Status::Value)
.collect();
}

### 返回闭包
- 闭包表现为trait，这意味着不能直接返回闭包。对于大部分需要返回trait的情况，可以使用实现了期望返回的trait的具体类型来替代函数的返回值。
- 因为他们没有一个可返回的具体类型，
- 不知道多少空间来存储闭包
fn returns_closure()->Box<dyn Fn(i32)->i32>{
    Box::new(|x|x+1)
}

## 宏
- 声明宏
- 使用宏
- 三种过程宏
    - 自定义宏#[derive]宏在结构体和枚举上指定通过derive属性添加的代码
    - 类属性attribute宏定义可用于任意项的自定义属性
    - 类函数宏看起来像函数不过作用于作为参数传递的token
## 宏和函数的区别
- 为其他代码而写的代码方式，元编程metaprogramming
- 宏只接受一个可变参数：用一个参数调用println!
- 编译器编译前展开，
- 可以在一个给定类型上实现trait
- 而函数不幸，因为函数是在运行时被调用，同时trait需要在编译时实现
- 宏定义复杂了，调用宏之前必须定义并将其引入作用域，而函数则可以在任何地方定义和调用

## 使用macro_rules!的声明宏用于通用元编程
- declarative macros
- 核心概念时声明宏允许我们编写一些类似于rust match表达式的代码，
- match 表达式时控制结构。其接收一个表达式，与表达式的结果进行模式进行比较
- 这种情况下，该值是传递给宏的rust源代码字面值，模式用于和传递给宏的源代码进行比较。
- 同时每个模式的相关代码则用于替换传递给宏的代码。所有这一切都发生在编译的时候。
- 可以使用macro_rules!来定义宏，
let v:Vec<u32> = vec![1,2,3];
#[macro_export]                                 //注解说明宏应该是可用的，如果没有这个注解，可能这个宏不会被引入
macro_rules! vec{                               // macro_rules! + 名称，所定义的宏并不带感叹号
    ($($x : expr), *) =>{                       //$通过替代代码捕获了符合括号内模式的值， * 零个或多个
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
- 一个vec!宏定义的简化版本
- 后面可以用macro关键字的声明宏
### 用于从属性生成代码的过程宏procedural macros
- 更像函数，
- 接受rust代码作为输入，
- 在这些代码上进行操作，然后产生另一些代码作为输出，而非ie像声明式宏那样匹配对应模式然后以另一部分替代当前代码
- 当创建过程宏时，其定义必须位于一种特殊类型的属于他们自己的crate中，
use proc_macro;
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream{}
- 包含一个函数，宏所处理的源代码组成了输入TokenStream,同时宏产生的代码时输出TokenStream
- 最后，函数上有一个属性，这个属性表明过程宏的类型，在同一个crate中可以有多种的过程宏

### 如何编写自定义derive宏
- 创建一个hello_macro crate，其包含名为HelloMacro的trait和关联函数hello_macro。
- 不同于让crate的用户为其每一个类型实现HelloMacro trait，
- 程序员可以通过正常调用：
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
#[derive(HelloMa)]
struct Pancakes;

fn main(){
    Pancakes::hello_macro();
}


pub trait HelloMacro{
    fn hello_macro();
}

- 我们无法为hello_macro函数提供一个能够打印实现了该trait的类型的名称的默认实现：rust没有反射的能力，因此其无法在运行时获取类型名，我们需要一个在运行时生成代码的宏。
- 下一步定义宏，必须在自己的crate中，在hello_macro目录下创建
- 如果改变hello_macro中定义的trait，同时必须改变hello_macro_derive中实现的过程式宏，这两个包需要同时发布，
- 如果要使用。同时添加，
- 为定义一个冲程宏
-proc_macro crate
是编译器用来读�和操作我们 Rust 代码的 API。
- syn crate ��符串中的 Rust 代码��成为一�可�操作的�据结�。 quote 则�
syn ��的�据结��过来传�� Rust 代码中。
- 当用户在一��型上指� #[derive(HelloMacro)] 时, hello_macro_derive ���
��调用。��在于我们已经使用 proc_macro_derive �其指���对
hello_macro_derive ��进行���� HelloMacro ,其�配� trait �,这是大�
�过程���的习惯。



### 类属性宏
- 它允许你创建新的属性，更为灵活，可以用于其他项包括函数
#[route(GET, "/“）]
fn index(){}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

### 类函数宏
��于 macro_rules! ,�们���更�活��
�,可��受未知�量的��。�而 macro_rules! ���使用�前 “使用
macro_rules! 的声明�用于�用元编程” �绍的��配的语��义
let sql = sql!(SELECT * FROM posts WHERE id=1);

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {