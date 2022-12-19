use std::env;
mod socket;
use socket::*;

use trial::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let role = socket::Role::decide(args[1].as_str());

    let host = "127.0.0.1";
    let port = 8080;

    let address = socket::create_ip_address(host, port);

    match role {
        Role::Server => {
            server_func(address);
        }
        Role::Client => {
            client_func(address);
        }
    }
}

mod trial {
    use std::net::TcpStream;
    use super::*;

    pub fn server_func(address: String) {
        let mut server = Server::new(address);
        loop {
            read(&mut server.tcp_stream);

            let msg = "thank you for your msg!";
            server.tcp_stream.send_msg(msg);
        }
    }

    pub fn client_func(address: String) {
        let mut client = Client::new(address);
        loop {
            let msg = "hoge";
            client.tcp_stream.send_msg(msg);

            read(&mut client.tcp_stream);
        }
    }

    fn read(tcp_stream: &mut TcpStream) {
        let mut msg = String::new();
        tcp_stream.read_msg(&mut msg);
        println!("received msg: {}", msg);
    }
}
