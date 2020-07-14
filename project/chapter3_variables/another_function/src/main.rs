fn main() {
    another_function(5);

    //expression
    let x = 5;
    let y ={
        let x = 3; 
        x + 2
    };
    println!("y= {}",y);
    println!("five()={}",five());

    if x < 9{
        println!("if branch.");
    }else{
        println!("else branch.");
    }
    //if + return
    let y = if x > 3{
        4
    }else{
        9
    };
    println!("y={}",y);

    //loop + return
    let mut i =1;
    let result = loop{
        i+=1;
        if i == 3{
            break i * 2;
        }
    };
    println!("result = {}",result);

    //while
    let mut num = 2;
    while num >0{
        num-=1;
    }
    println!("num = {}", num);

    //for
    let a = [1,2,34,5,6,7];
    for  ele in a.iter(){
        println!("ele = {}", ele);
    }

    for ele in (1..4).rev(){
        println!("ele2={}",ele);
    }
}
fn another_function(x :i32){
    println!("x={}",x);
}
//function 返回值
fn five() -> i32{
    5//expression
}