extern crate serde;
extern crate serde_json;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind};


#[derive(Serialize, Deserialize, Debug)]
struct Resource {
    uuid: String,
    name: String,
    address: String,
    roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Inventory {
    resources: Vec<Resource>,
}

pub fn get_hosts() -> Result<(Vec<String>), Error> {
    match env::home_dir() {
        Some(path) => {
            let full_path = &format!("{}/{}", path.display(), ".carbon_config/inventory.json");
            // println!("Inventory File Exists {}", full_path);
            if Path::new(full_path).exists() {
                let mut file = File::open(full_path).unwrap();
                let mut data = String::new();
                file.read_to_string(&mut data).expect("Unable to open");
                // println!("Raw String Data: {}", data);
                let inv: Inventory = serde_json::from_str(&data).unwrap();
                let resource_list: Vec<Resource> = inv.resources;
                let hosts: Vec<String> = resource_list.iter().map(|r| r.address.clone()).collect();
                Ok(hosts)
            } else {
                println!("Could not open file");
                Err(Error::new(ErrorKind::Other, "oh no!"))
            }
        }
        None => {
            println!("Impossible to get your home dir!");
            Err(Error::new(ErrorKind::Other, "oh no!"))
        }
    }
}