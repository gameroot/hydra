#[macro_use]
extern crate serde_derive;
extern crate shrust;

use shrust::{Shell, ShellIO};
use std::io::prelude::*;

mod roles;
mod config;
mod inventory;
mod remote;

fn main() {
    let v = Vec::new();
    let mut shell = Shell::new(v);
    shell.new_command_noargs("inventory", "Show local network inventory.", |io, _| {
        try!(writeln!(io, "Showing local network inventory."));
        inventory::load_from_json();
        Ok(())
    });
    shell.new_command_noargs("ping", "Ping known hosts.", |io, _| {
        remote::ping_remote_hosts();
        try!(writeln!(io, "Pinging known hosts."));
        Ok(())
    });
    shell.new_command("add_host_to_inventory", "Add host to inventory.", 1, |io, v, s| {
        try!(writeln!(io, "Pushing new host {} to inventory.", s[0]));
        v.push(s[0].to_string());
        Ok(())
    });
    shell.new_command_noargs("refresh_inventory", "Write JSON inventory to RocksDB.", |io, _| {
        try!(writeln!(io, "Hello World !!!"));
        inventory::refresh();
        Ok(())
    });
    shell.new_command_noargs("init", "Create config directory if it doesn't exist.", |io, _| {
        try!(writeln!(io, "Hello World !!!"));
        config::init();
        Ok(())
    });
    shell.new_command_noargs("profile_hosts", "Get remote host system profiles.", |io, _| {
        try!(writeln!(io, "Getting remote host system profiles."));
        remote::profile_hosts();
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}

 