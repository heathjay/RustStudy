fn main() {
    //no insert 
    let v: Vec<i32> = Vec::new();
    //insert init
    let v1 = vec![1,2,3];

    let mut v2 = Vec::new();
    //insert
    v2.push(1);
    v2.push(2);
    //read by reference + index
    let third:&i32 = &v1[2];
    println!("third:{}",third);
    //read by get
    match v1.get(2){
        Some(third) => println!("The third elem is {}",third),
        None => println!("there is no elem"),
    }

    for i in &v1{
        println!("v1={}",i);
    }

   
    //mut vector and mutable
    let mut v3 = vec![1,2,3,4];
    for i in &mut v3{
        println!("v3={}",*i+1);
    }

    //enum : different type
    //#[derive(debugging)]
    enum SpecialItem{
        Integer(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpecialItem::Integer(3),
        SpecialItem::Float(3.3),
        SpecialItem::Text(String::from("sjs")),
    ];

    for i in &row{
        match i{
            SpecialItem::Integer(i) => println!("integer"),
            SpecialItem::Float(i) => println!("float"),
            SpecialItem::Text(state) => println!("string"), 
        }
    }
}

