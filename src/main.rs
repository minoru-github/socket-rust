use std::env;
mod socket;
use socket::*;

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
