use crate::configure::Target;
use crate::clients::client::Client;
use reqwest;

pub struct GoogleDomainsClient {

}

impl Client for GoogleDomainsClient {
    fn update(target: &Target) {
        // https://username:password@domains.google.com/nic/update?hostname=subdomain.yourdomain.com&myip=1.2.3.4

        let client = reqwest::Client::new();
        let req = client.post("https://domains.google.com/nic/update")
            .header("User-Agent", "Chrome/41.0 ")
            .basic_auth(&target.username, Some(&target.password))
            .query(&[("hostname", target.hostname.as_str())])
            .send();

        println!("{:?}", req);
    }
}
