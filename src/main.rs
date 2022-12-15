use std::{
    env,
    io::BufReader,
    io::{BufRead, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let role = match args[1].as_str() {
        "s" | "server" => {
            println!("|| server ||");
            Role::Server
        }
        "c" | "client" => {
            println!("|| client ||");
            Role::Client
        }
        _ => {
            panic!("invalid arg.");
        }
    };

    let host = "127.0.0.1";
    let port = 8080;

    let mut socket = Socket::new(host, port, role);

    match role {
        Role::Server => {
            let mut msg = String::new();
            socket.read(&mut msg);
            println!("{}", msg);
        }
        Role::Client => {
            socket.send("hoge");
        }
    }
}

#[derive(Clone, Copy)]
enum Role {
    Server,
    Client,
}

#[derive(Debug)]
struct Socket {
    stream: TcpStream,
}

impl Socket {
    fn new(host: &'static str, port: usize, role: Role) -> Self {
        let addr = format!("{}:{}", host, port);

        let stream = match role {
            Role::Server => {
                let tcp_listener = TcpListener::bind(addr.clone()).expect("can't bind.");
                let (tcp_stream, _) = tcp_listener.accept().expect("can't accept.");
                tcp_stream
            }
            Role::Client => TcpStream::connect(addr.clone()).expect("can't connet."),
        };
        Socket {
            stream,
        }
    }

    fn read(&mut self, msg: &mut String) {
        let mut reader = BufReader::new(&mut self.stream);
        reader.read_line(msg).expect("can't receive.");
    }

    fn send(&mut self, msg: &str) {
        self.stream.set_nonblocking(false).expect("out of service.");
        println!("succeeded in connecting server.");

        let msg = msg.as_bytes();
        self.stream.write_all(msg).expect("can't send msg.");
    }
}
