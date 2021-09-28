use std::{env::{args}, net::{ToSocketAddrs}};

use rust_port_scan::{input::Parameter, output::Message, scan::scan_ports};

fn main() -> () {
    let param = Parameter::unsafe_parse(&mut args());
    let addrs = param.target.to_socket_addrs().unwrap().next().unwrap();
    let opend_ports = scan_ports(&addrs);
    let message = Message { ports: opend_ports };
    println!("{}", message);
}
