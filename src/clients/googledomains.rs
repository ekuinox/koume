use crate::configure::Target;
use crate::clients::client::Client;
use reqwest;
use std::net::*;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

pub struct GoogleDomainsClient {

}

impl Client for GoogleDomainsClient {
    fn update(target: &Target) {
        // https://username:password@domains.google.com/nic/update?hostname=subdomain.yourdomain.com&myip=1.2.3.4


        let client = reqwest::Client::new();

        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
        
        let response = resolver.lookup_ip(target.hostname.as_str()).unwrap();

        let ip_addr: IpAddr = client.get("https://domains.google.com/checkip").send().unwrap().text().unwrap().parse().unwrap();

        for address in response.iter() {
            if ip_addr == address {
                println!("already registered {} to {}", ip_addr, target.hostname);
                return
            }
        }

        let request = client
            .get("https://domains.google.com/nic/update")
            .basic_auth(&target.username, Some(&target.password))
            .query(&[("hostname", target.hostname.as_str())])
            .build()
            .unwrap();

        match client.execute(request) {
            Ok(mut response) => {
                if response.status() != 200 {
                    // bad request
                    return
                }
                println!("{}", response.text().unwrap());
            },
            Err(error) => {
                println!("{:?}", error)
            }
        }
    }
}
