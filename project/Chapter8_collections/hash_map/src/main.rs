fn main() {
    //use crate
    use std::collections::HashMap;
    //new
    let mut scores = HashMap::new();
    //insert
    scores.insert("b",9);
    scores.insert("a",10);


    //create + collection
    let team = vec![String::from("blue"),String::from("apple")];
    let ini_scores = vec![11,23];

    let scores:HashMap<_,_> = team.iter().zip(ini_scores.iter()).collect();

    //ownership:string:move
    let field_name=String::from("hello");
    let field_val = String::from("world");
    let mut map = HashMap::new();
    map.insert(field_name,field_val);
    //ownership::interger : copy
    let field_name=12;
    let field_val = 33;
    let mut map = HashMap::new();
    map.insert(field_name,field_val);

    //get
    let mut map = HashMap::new();
    map.insert(String::from("hello"),10);
    map.insert(String::from("world"),20);
    let item = String::from("hello");
    let scores = map.get(&item);
    let i = 10;
    let so = match scores{
        Some(&i) => 10,
        None => 0,
    };

    //for
    for (key,val) in &map{
        println!("{}:{}",key,val);
    }

    //update: key
    map.insert(String::from("hello"),20);
    println!("{:?}",map);
    //exist?_:insert
    //entry().or_insert()
    map.insert(String::from("blue"),22);
    map.entry(String::from("yellow")).or_insert(50);
    map.entry(String::from("blue")).or_insert(50);

    println!("{:?}",map);

    //update val based on old val
    //entry().or_insert():=The or_insert method actually returns a mutable reference(&mut V) to the value for this key
    let text = "hello world and wonderful world hello";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",map);

    let mut v = Vec::new();
    v.push(1);
    v.push(3);
    v.push(2);
    v.sort();
    for i in v{
        println!("{}",i);
    }
}
