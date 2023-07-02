use core::net::Ipv4Addr;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use libarp::interfaces::MacAddr;

const ARP_CACHE: &'static str = "/proc/net/arp";

pub fn generate_cache() -> HashMap<Ipv4Addr, MacAddr> {
    let mut cache: HashMap<Ipv4Addr, MacAddr> = HashMap::new();

    return cache;
}

pub fn update_cache(cache: &mut HashMap<Ipv4Addr, MacAddr>) {
    let arp_cache = File::open(ARP_CACHE)?;
    let mut arp_cache_reader = BufReader::new(arp_cache).lines();

    // Ignore the first line, which contains the header
    let _header = arp_cache_reader.next();

    while let Some(line) = arp_cache_reader.next() {
        let line = line?;
        let splits: Vec<&str> = line.split_whitespace().collect();

        if splits.len() < 6 {
            return;
        }

        let ip4: Ipv4Addr = splits[0].to_string().parse().unwrap();
        let mac: MacAddr = splits[3].to_string().parse().unwrap();

        cache.entry(ip4).or_insert(mac);
    }
}
