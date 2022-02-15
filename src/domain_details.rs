use std::collections::HashSet;
use std::net::IpAddr;

pub struct DomainDetails {
    name: String,
    ipv4: HashSet<IpAddr>,
    ipv6: HashSet<IpAddr>,
}

impl Default for DomainDetails {
    fn Default() -> Self {
        DomainDetails {
            name: String::from(""),
            ipv4: HashSet::new(),
            ipv6: HashSet::new(),
        }
    }
}
