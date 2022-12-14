#![allow(unused)]
use std::{
    env,
    io::Error,
    io::{BufRead, BufWriter, ErrorKind, Write},
    io::{BufReader, Result},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let mode = args[1].as_str();

    let host = "localhost";
    let port = 3000;

    let socket = Socket::new(host, port);
    let tcp_strem = socket.connect().expect("can't connect");

    match mode {
        "read" => {
            socket.read(tcp_strem);
        }
        "send" => {
            socket.send(tcp_strem, "hoge");
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
    addr_ipv4: Option<SocketAddr>,
}

impl Socket {
    fn new(host: &'static str, port: usize) -> Self {
        let host_and_port = format!("{}:{}", host, port);
        let mut addr = host_and_port.to_socket_addrs().unwrap();

        let addr_ipv4 = if let Some(addr) = addr.find(|&x| x.is_ipv4()) {
            Some(addr)
        } else {
            None
        };

        Socket {
            host,
            port,
            addr_ipv4,
        }
    }

    fn connect(&self) -> Result<TcpStream> {
        if let Some(addr) = self.addr_ipv4 {
            TcpStream::connect(addr)
        } else {
            let error = Error::from(ErrorKind::AddrNotAvailable);
            Err(error)
        }
    }

    fn read(&self, tcp_stream: TcpStream) {
        let mut reader = BufReader::new(&tcp_stream);

        let mut msg = String::new();
        reader.read_line(&mut msg).expect("can't receive");
        println!("{}", msg);
    }

    fn send(&self, tcp_stream: TcpStream, comment: &str) {
        let mut writer = BufWriter::new(&tcp_stream);

        let msg = format!("message: {}", comment);
        writer.write(msg.as_bytes()).expect("can't write");
        writer.flush().unwrap();
    }
}
