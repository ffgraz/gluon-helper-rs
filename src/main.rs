#![feature(ip_in_core)]

mod arp;

use arp::ArpResolver;
use core::net::Ipv4Addr;

fn main() {
    let mut resolver = ArpResolver::new();
    println!("Hello, world!");
}
