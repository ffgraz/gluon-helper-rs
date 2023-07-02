#![feature(ip_in_core)]

use core::net::Ipv4Addr;

use arp::ArpResolver;

mod arp;

fn main() {
    let mut resolver = ArpResolver::new();
    resolver.ip4_to_mac("127.0.0.1".to_string().parse().unwrap());
    println!("Hello, world!");
}
