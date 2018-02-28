//! Manages creating and reading from the disk cache
use std::fs;
use std::env;
use std::path::Path;

pub fn create_config_dir() {
    //! Creates application config at ~/.carbon_config if it doesn't exist.
    match env::home_dir() {
        Some(path) => {
            let full_path = &format!("{}/{}", path.display(), ".carbon_config");
            if Path::new(full_path).exists() {
                println!("Config Directory Exists");
            } else {
                println!("Config directory does not exist.");
                println!("Creating config dir at : {}", full_path);
                match fs::create_dir(full_path) {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(_) => {}
                }
            }
        }
        None => println!("Impossible to get your home dir!"),
    }
}
