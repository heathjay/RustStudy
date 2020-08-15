use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message{
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool{
        workers:Vec<Worker>,
        sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
struct Worker{
    id:usize,
    thread:Option<thread::JoinHandle<()>>,
}

impl Worker{
        fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
            let thread = thread::spawn(move ||{
                loop{
                    let message = receiver.lock().unwrap().recv().unwrap();

                    match message{
                        Message::NewJob(job) =>{
                            println!("worker {} got a job; executing",id);
                            job();
                        },
                        Message::Terminate=>{
                            println!("worker {} was told to terminate", id);
                            break;
                        },
                    }
                }
            
            });
            Worker{
                id,
                thread: Some(thread),
            }
        }
}


impl ThreadPool{
    ///创建线程池
    /// 
    /// 线程池中线程的数量
    /// 
    /// #Panics
    /// 
    /// `new` 函数在size为0会panic
    /// 
    /// 

    pub fn new(size:usize) -> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender,receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }


        ThreadPool{
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f:F)
        where F: FnOnce() + Send+ 'static{
        let job = Box::new(f);
//        self.sender.send(job).unwrap();
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        println!("sending terminate message to all workers.");
        for _ in &mut self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("shutting down all");
        for worker in &mut self.workers{
            println!("shutring down worker {}", worker.id);
            //使用take把所有权从worker里面弄出来
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
            //worker.thread.join().unwrap();
        }
    }
}



/*
pub struct ThreadPool{
//    threads:Vec<thread::JoinHandle<()>>,
    workers:Vec<Worker>,
    //通道
    sender: mpsc::Sender<Job>,
}
//struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;
/*
struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}
*/


struct Worker{
    id:usize,
    thread:Option<thread::JoinHandle<()>>,
}
impl Worker{
//    fn new(id:usize,receiver:mpsc::Receiver<Job>) -> Worker{
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move ||{
            loop{
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("worker {} get a job; executing",id);
                job();
            }
        
        });
        Worker{
            id,
//            thread,
            thread: Some(thread),
        }
    }
}

impl ThreadPool{
    ///创建线程池
    /// 
    /// 线程池中线程的数量
    /// 
    /// #Panics
    /// 
    /// `new` 函数在size为0会panic
    /// 
    /// 
    /*
    pub fn new(size:usize) -> ThreadPool{
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size{
            //create some threads and store thm in the vector
        }


        ThreadPool{
            threads
        }
    }
    */

    pub fn new(size:usize) -> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender,receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size{
            //create some threads and store thm in the vector
//            workers.push(Worker::new(id,receiver));
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }


        ThreadPool{
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f:F)
        where F: FnOnce() + Send+ 'static{
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        for worker in &mut self.workers{
            println!("shutring down worker {}", worker.id);
            //使用take把所有权从worker里面弄出来
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
            //worker.thread.join().unwrap();
        }
    }
}


*/