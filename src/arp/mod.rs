// Library to resolve IPs to MAcs
// First checks for IP in /proc/arp, then tries to resolve using arp-toolkit
// Keeps cache in memory

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::Duration;

use libarp::{client::ArpClient, interfaces::MacAddr};

use arp_cache::update_cache;

mod arp_cache;

pub struct ArpResolver {
    arp_cache: HashMap<Ipv4Addr, MacAddr>,
    arp_client: ArpClient,
}

// TODO: make the individual resolves (arpcache, arpresolve) traits of resolver?

const ONE_SECOND: Duration = Duration::new(1, 0);

impl ArpResolver {
    pub fn new() -> Self {
        dbg!("Before kaboom");
        let arp_cache = HashMap::new();
        dbg!("Before kaboom");
        let arp_client = ArpClient::new();

        if let std::io::Result::Err(err) = arp_client {
            dbg!(&err);

            panic!("Error creating arp client: {}", err);
        }

        Self {
            arp_cache,
            arp_client: arp_client.unwrap(),
        }
    }

    pub async fn ip4_to_mac(&mut self, ip_addr: Ipv4Addr) -> Option<MacAddr> {
        return match self.arp_cache.get(&ip_addr) {
            Some(mac) => Some(*mac),
            None => {
                return match self.arp_cache.get(&ip_addr) {
                    Some(mac) => Some(*mac),
                    None => {
                        update_cache(&mut self.arp_cache);
                        let result = self.arp_client.ip_to_mac(ip_addr, Some(ONE_SECOND));
                        match result.await {
                            Ok(mac) => {
                                self.arp_cache.insert(ip_addr, mac);
                                Some(mac)
                            }
                            Err(_err) => None,
                        }
                    }
                }
            }
        };
    }
}
