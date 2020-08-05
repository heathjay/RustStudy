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