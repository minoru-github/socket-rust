use std::{
    io::Error,
    io::ErrorKind,
    io::Result,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
};

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Debug)]
struct Stream {
    host: String,
    port: String,
    addr_ipv4: Option<SocketAddr>,
}

impl Stream {
    fn new(host: String, port: String) -> Self {
        let host_and_port = format!("{}:{}", host, port);
        let mut addr = host_and_port.to_socket_addrs().unwrap();

        let addr_ipv4 = if let Some(addr) = addr.find(|&x| x.is_ipv4()) {
            Some(addr)
        } else {
            None
        };

        Stream {
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
}
