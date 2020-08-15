use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

use std::thread;
use std::time::Duration;

fn main() {
    //bind返回result<T,E>
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //返回一个迭代器
    //流代表一个客户端和服务段之间的连接
    for stream in listener.incoming(){
        // unwrap调用，出现问题就退出终止程序
        let stream = stream.unwrap();

        println!("connection established!");

        handle_connection(stream);
    }
}
/*
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;512];   //开辟缓存512字节，[起始下标，大小]
    stream.read(&mut buffer).unwrap();
   // println!("requests:{}!", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string("./hello.html").unwrap();
//    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
*/
/*
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;512];   //开辟缓存512字节，[起始下标，大小]
    stream.read(&mut buffer).unwrap();
// println!("requests:{}!", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get){
        let contents = fs::read_to_string("./hello.html").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else{
        let status_line ="HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}{}",status_line,contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
*/

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