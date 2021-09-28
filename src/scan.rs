use std::net::{SocketAddr, TcpStream};

pub fn scan(target: &SocketAddr) -> bool {
    let result = TcpStream::connect(target);
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn scan_ports(target: &SocketAddr) -> Vec<u16> {
    let mut acc = Vec::new();
    for port in 0..=65535 {
        let addr = SocketAddr::new(target.ip(), port);
        let is_open = scan(&addr);
        if is_open { acc.push(port); }
    };
    acc
}
