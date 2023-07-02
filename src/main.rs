#![feature(ip_in_core)]

mod arp;

use arp::ip4_to_mac;
use core::net::Ipv4Addr;

fn main() {
    println!("Hello, world!");
}
