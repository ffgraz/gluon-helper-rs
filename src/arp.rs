// Library to resolve IPs to MAcs
// First checks for IP in /proc/arp, then tries to resolve using arp-toolkit
// Keeps cache in memory

mod arp_cache;
use arp_cache::update_cache;
use core::net::Ipv4Addr;
use std::collections::HashMap;
use std::io::Error;

use futures::executor::block_on;
use libarp::{client::ArpClient, interfaces::MacAddr};

struct ArpResolver {
    arp_cache: HashMap<Ipv4Addr, MacAddr>,
    arp_client: ArpClient,
}

impl ArpResolver {
    fn new() -> Self {
        Self {
            arp_cache: HashMap::new().
            client: ArpClient::new().unwrap(),
        }
    }
}

pub async fn ip4_to_mac(ip_addr: Ipv4Addr) -> Result<MacAddr, Error> {
    let mut client = ArpClient::new().unwrap();

    let result = client.ip_to_mac(ip_addr, None);
    return result.await.unwrap();
}
