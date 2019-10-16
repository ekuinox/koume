use crate::configure::Target;
use crate::clients::client::Client;
use reqwest;

pub struct GoogleDomainsClient {

}

impl Client for GoogleDomainsClient {
    fn update(target: &Target) {
        // https://username:password@domains.google.com/nic/update?hostname=subdomain.yourdomain.com&myip=1.2.3.4

        let client = reqwest::Client::new();
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
