
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("number guess!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("secret_number:{}",secret_number);
    let mut guess = String::new();//create a variable, mut is used to create a mutable variable
    /**
     * 
     * let foo = 5;//immutable
     * 
    */
    // loop{
    //     println!("Please enter a number!");
    //     io::stdin().read_line(&mut guess).expect("failed to read line");
    //     println!("you guess :{}",guess);  
    //     let guess:u32 = match guess.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
       
    //     match guess.cmp(&secret_number){
    //     Ordering::Less =>println!("small"),
    //     Ordering::Equal =>{
    //         println!("good!");
    //         break;
    //     } 
    //     Ordering::Greater => println!("greater!"),
    //     }
    // }
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("This result is {}", result);
}
