use std::{fmt::format, fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connections(stream);
        });
    }

    println!("====== Shutting down ======")
}

fn handle_connections(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 ok", "./src/webpages/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 ok", "./src/webpages/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./src/webpages/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContents-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    
}
