//多线程web项目

/*
- 在socket上监听TCP连接
- 解析少量的HTTP请求
- 创建一个合适的HTTP请求
*/

use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream}, time::Duration,thread
};

use webApp::ThreadPool;

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     // eprint!("Connection established!");



         
    //     // handle_connection(stream);

    //     // //为每个连接创建一个线程
    //     // //缺点：黑客使用DOS攻击，每一个都会瘫掉  
    //     // thread::spawn(||{
    //     //     handle_connection(stream);
    //     // });



    // }


    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);
    for strem in listener.incoming(){
        let stream = strem.unwrap();
        pool.execute(||{
            handle_connection(stream);
        });
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 9999];
    //512字节的缓存区
    stream.read(&mut buffer).unwrap();
    // eprint!("Request:{}",String::from_utf8_lossy(&buffer[..]))
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    }else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    }; 

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


}
