//! Disk cache located at ~/.carbon_config
//! It contains the "hosts inventory" JSON file.
pub mod disk_cache; 


pub fn init() {
  disk_cache::create_config_dir()
}