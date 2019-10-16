extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod configure;
mod clients;

use configure::Configures;
use clients::client::Client;
use clients::googledomains::GoogleDomainsClient;

fn main() {

    match Configures::import_from_file("./conf.json") {
        Ok(configures) => {
            for target in &configures.targets["googledomains"] {
                GoogleDomainsClient::update(&target);
            }
        }
        Err(error) => println!("{:?}", error)
    }
}
