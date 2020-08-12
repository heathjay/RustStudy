use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {

    let handle = thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    let v = vec![1,2,3];
    let handle = thread::spawn(move ||{
        println!("here is a vector:{:?}", v);
    });
    handle.join().unwrap();


    let (tx, rx) = mpsc::channel();
    //多个生产者
    //单个消费者multiple producer single consumer
    thread::spawn(move||{
        let val = String::from("hi");
        tx.send(val).unwrap();
        //所有权进行转移    
        //println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("got:{}",received);


    let (tx,rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    //move 所有权转移
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
    //多个生产者

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx{
        println!("got : {}", received);
    }
}
