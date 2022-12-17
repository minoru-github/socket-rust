use socket::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let role = socket::Role::decide(args[1].as_str());

    let host = "127.0.0.1";
    let port = 8080;

    let address = socket::create_ip_address(host, port);

    match role {
        Role::Server => {
            let mut server = Server::new(address);
            let mut msg = String::new();
            server.tcp_stream.read(&mut msg);
            println!("{}", msg);
        }
        Role::Client => {
            let mut client = Client::new(address);
            client.tcp_stream.send("hoge");
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

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Role {
        Server,
        Client,
    }

    impl Role {
        pub fn decide(arg: &str) -> Self {
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
    }

    #[test]
    fn test_decide_to_role() {
        let arg = "c";
        let role = Role::decide(arg);
        assert_eq!(role, Role::Client);

        assert_eq!(Role::decide("s"), Role::Server);
        assert_eq!(Role::decide("client"), Role::Client);
        assert_eq!(Role::decide("server"), Role::Server);
    }

    pub trait ReadWrite {
        fn read(&mut self, msg: &mut String);
        fn send(&mut self, msg: &str);
    }

    impl ReadWrite for TcpStream {
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

    pub struct Server {
        pub tcp_stream: TcpStream,
    }
    impl Server {
        pub fn new(address: String) -> Self {
            let tcp_listener = TcpListener::bind(address).expect("can't bind.");
            let (tcp_stream, _) = tcp_listener.accept().expect("can't accept.");
            Server { tcp_stream }
        }
    }

    pub struct Client {
        pub tcp_stream: TcpStream,
    }
    impl Client {
        pub fn new(address: String) -> Self {
            let tcp_stream = TcpStream::connect(address).expect("can't connet.");
            Client { tcp_stream }
        }
    }
}
