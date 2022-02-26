use std::env;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage {} hostname", args[0]);
        std::process::exit(1);
    }
    let hostname = &args[1];

    // Construct a new Resolver with default configuration options
    // Default gets IPv4 and then IPv6
    let mut opts = ResolverOpts::default();
    match opts.ip_strategy {
        LookupIpStrategy::Ipv4Only => println!("IPv4 only"),
        LookupIpStrategy::Ipv6Only => println!("IPv6 only"),
        LookupIpStrategy::Ipv4AndIpv6 => println!("IPv4 and v6"),
        LookupIpStrategy::Ipv6thenIpv4 => println!("IPv6 then 4"),
        LookupIpStrategy::Ipv4thenIpv6 => println!("IPv4 then 6"),
    }
    // then change to find both
    opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;

    let resolver = Resolver::new(ResolverConfig::default(), opts).unwrap();

    let response = resolver.lookup_ip(hostname);
    match response {
        Ok(resp) => {
            for a in resp.iter() {
                println!("addr {}", a);
            }
        }
        Err(err) => {
            println!("err {}", err);
            std::process::exit(2)
        }
    }
    std::process::exit(0);
}
