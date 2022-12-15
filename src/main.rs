#![allow(unused)]
use std::{
    env,
    io::Error,
    io::{BufRead, BufWriter, ErrorKind, Write},
    io::{BufReader, Result},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let mode = args[1].as_str();

    let host = "127.0.0.1";
    let port = 8080;

    let socket = Socket::new(host, port);

    match mode {
        "read" => {
            let mut msg = String::new();
            socket.read(&mut msg);
            println!("{}", msg);
        }
        "send" => {
            socket.send("hoge");
        }
        _ => {
            panic!("invalid arg")
        }
    }
}

#[derive(Clone, Debug)]
struct Socket {
    host: &'static str,
    port: usize,
    addr: String,
}

impl Socket {
    fn new(host: &'static str, port: usize) -> Self {
        let addr = format!("{}:{}", host, port);
        Socket { host, port, addr }
    }

    fn read(&self, msg: &mut String) {
        let tcp_listener = TcpListener::bind(&self.addr).expect("can't bind.");
        let (mut stream, _) = tcp_listener.accept().expect("can't accept.");

        self.read_stream(&mut stream, msg);
    }

    fn read_stream(&self, stream: &mut TcpStream, msg: &mut String) {
        let mut reader = BufReader::new(stream);
        reader.read_line(msg).expect("can't receive.");
    }

    fn send(&self, msg: &str) {
        let mut tcp_stream = TcpStream::connect(&self.addr).expect("can't connet.");

        tcp_stream.set_nonblocking(false).expect("out of service.");
        println!("// connect server // ");
        let msg = msg.as_bytes();

        tcp_stream.write_all(msg).expect("can't send msg.");

        let mut writer = BufWriter::new(&tcp_stream);
    }
}
