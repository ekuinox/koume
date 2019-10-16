use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Target {
    pub hostname: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configures {
    pub targets: HashMap<String, Vec<Target>>,
}

impl Configures {
    pub fn import_from_file<'a>(path: &'a str) -> Result<Configures, serde_json::Error> {
        serde_json::from_str::<Configures>(read_to_string(path).unwrap().as_str())
    }
}
