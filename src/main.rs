use socket::*;
use std::env;
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();

    let role = socket::decide_to_role(args[1].as_str());

    let host = "127.0.0.1";
    let port = 8080;

    let address = socket::create_ip_address(host, port);

    match role {
        Role::Server => {
            let mut server = TcpStream::new(address, role);
            let mut msg = String::new();
            server.read(&mut msg);
            println!("{}", msg);
        }
        Role::Client => {
            let mut client = TcpStream::new(address, role);
            client.send("hoge");
        }
    }
}

mod socket {
    use std::{
        io::BufReader,
        io::{BufRead, Write},
        net::{TcpListener, TcpStream},
    };

    pub fn create_ip_address(host: &'static str, port: usize) -> String {
        // TODO IPV4,IPV6以外はエラーにする
        format!("{}:{}", host, port)
    }

    #[test]
    fn test_create_ip_address() {
        let host = "127.0.0.1";
        let port = 8080;
        let address = create_ip_address(host, port);
        assert_eq!(address, "127.0.0.1:8080");

        assert_eq!(create_ip_address("0.0.0.0", 1), "0.0.0.0:1");

    }

    pub fn decide_to_role(arg: &str) -> Role {
        match arg {
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
        }
    }

    #[test]
    fn test_decide_to_role() {
        let arg = "c";
        let role = decide_to_role(arg);
        assert_eq!(role, Role::Client);

        assert_eq!(decide_to_role("s"), Role::Server);
        assert_eq!(decide_to_role("client"), Role::Client);
        assert_eq!(decide_to_role("server"), Role::Server);
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Role {
        Server,
        Client,
    }

    pub trait Stream {
        fn new(address: String, role: Role) -> Self;
        fn read(&mut self, msg: &mut String);
        fn send(&mut self, msg: &str);
    }

    impl Stream for TcpStream {
        fn new(address: String, role: Role) -> Self {
            match role {
                Role::Server => {
                    let tcp_listener = TcpListener::bind(address).expect("can't bind.");
                    let (tcp_stream, _) = tcp_listener.accept().expect("can't accept.");
                    tcp_stream
                }
                Role::Client => TcpStream::connect(address).expect("can't connet."),
            }
        }
        fn read(&mut self, msg: &mut String) {
            let mut reader = BufReader::new(self);
            reader.read_line(msg).expect("can't receive.");
        }
        fn send(&mut self, msg: &str) {
            self.set_nonblocking(false).expect("out of service.");
            println!("succeeded in connecting server.");

            let msg = msg.as_bytes();
            self.write_all(msg).expect("can't send msg.");
        }
    }
}
