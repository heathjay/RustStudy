use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

use std::thread;
use std::time::Duration;
use chapter28webserverMultiThread::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    //两个请求后退出循环
    for stream in listener.incoming().take(2){
        let stream = stream.unwrap();
        /*
        thread::spawn(||{
            handle_connection(stream);
        });
        */
        pool.execute(||{
            handle_connection(stream);
        });
    }
    println!("shutting down");
}


fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;512];   //开辟缓存512字节，[起始下标，大小]
    stream.read(&mut buffer).unwrap();
// println!("requests:{}!", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}",status_line,contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}