unsafe fn dangerous(){}

static HELLO_WORLD : &str = "hello, world";
extern "C"{
    fn abs(input:i32) -> i32;
}

static mut COUNTER:u32 = 0;
fn add_to_count(inc:u32){
    unsafe{
        COUNTER += inc;
    }
}

fn main() {
    println!("Hello, world!");
    unsafe{
        dangerous();
    }

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    assert_eq!(a,&mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    unsafe{
        println!("absolute value of -3 according to c: {}", abs(-3));
    }
    println!("name is {}", HELLO_WORLD);

    add_to_count(3);
    unsafe{
        println!("{}",COUNTER);
    }
}
