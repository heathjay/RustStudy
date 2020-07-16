fn main() {
    //string create
    let s = String::new();
    let data = "abc";
    let s1 = data.to_string();
    let s2 = "dc".to_string();
    let s3 = String::from("bs");
    let s = String::from("😊");
    println!("{}",s);

    //update
    //push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}",s);

    //test ownership
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("{}",s2);
    //push单个字符
    s.push('l');

    //+ = fn add(self, s: &str) -> String {
    let s1 = String::from("hello");
    let s2 = String::from(",world");
    //let s4 = &s1 + &s2;//出错，必须string类型
    let s3 = s1 + &s2;//s1 无效后面
    println!("{}",s3);
    let s4 = s3 + "-" + &s2 + "+" + &s2;
    println!("{}",s4);

    //format
    //ownership 不会变动
    let s5 = format!("{}-{}-{}",s4,s2,s2);
    println!("{}",s5);

    let len = s5.len();

    //chars : 来进行遍历
    for c in "hello".chars() { 
        println!("{}", c);
    }
}
