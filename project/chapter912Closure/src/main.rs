use std::thread;
use std::time::Duration;

//闭包结构体。必须要有类型，Fn参数和输出结果类型
struct Cacher<T> 
    where T : Fn(u32) -> u32
{
    calculation : T,
    //如果cacher的代码请求闭包的结果，
    //在执行闭包前，value是none，
    //如果使用cacher的代码请求闭包的结果，这时将会执行闭包将结果存在some中，接着在请求的直接返回存在some成员中的结果
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T : Fn(u32) -> u32
{
    fn new (calculation: T) -> Cacher<T>{
        Cacher{
            calculation,
            value:None,
        }
    }

    fn value(&mut self, arg: u32) -> u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout_2(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z==x;
    let y  = 4;
    assert!(equal_to_x(y));
}
//一个用来代替假想计算的函数，他大约执行两秒钟
fn simulated_expensive_calculation(intensity: u32)-> u32{
    println!("calculating slowly....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32){
    //函数重构
    //let expensive_result = simulated_expensive_calculation(intensity);
    

    //闭包使用
    let expensive_closure = |num|{
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25{
        println!("today, do {} pushups!", expensive_closure(intensity));
        println!("tomorrow, do {} situps!",expensive_closure(intensity));

        //println!("today, do {} pushups!", expensive_result);
        //println!("tomorrow, do {} situps!",expensive_result);
    }else{
        if random_number == 3{
            println!("take a break today! remember to stay hydrated!");
        }else{
            //println!("today, run for {} minutes!", expensive_result);
            println!("today, run for {} minutes!",expensive_closure(intensity));
        }
    }
}


fn generate_workout_2(intensity:u32, random_number: u32){
    let mut expensive_result = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity > 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }else{
        if random_number == 3{
            println!("Take a break today ! Remeber to stay hydrated!");
        }else{
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}