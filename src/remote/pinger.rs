extern crate ssh2;
extern crate serde_json;
use self::ssh2::Session;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;

use inventory;
use remote::profiler as profiler;

pub fn ping_remote_hosts() {
  match inventory::get_hosts() {
    Err(reason) => 
      println!("failed to get hosts"),
    Ok(hosts) =>
      for host in hosts {
        let handler = thread::spawn(move || {
            ping_host(&host);
        });
      },
  }
}

// Change to struct input - inventory::Resource
// TODO: Remove hardcoded name duh.
fn ping_host(host: &str){
  let tcp = TcpStream::connect(host).unwrap();
  let mut sess = Session::new().unwrap();
  sess.handshake(&tcp).unwrap();

  // after changing to struct
  // let ssh_user: String = resource.ssh_user
  let ssh_user: String = "jerome".to_string(); // temp
  sess.userauth_agent(&ssh_user).unwrap();
  let mut channel = sess.channel_session().unwrap();
  channel.exec(".hydra/agent").unwrap();
  let mut s = String::new();
  channel.read_to_string(&mut s).unwrap();
  let prof: profiler::SystemProfile = serde_json::from_str(&s).unwrap();
  match(s.contains("hostname")) {
    true => println!("node {} up", host),
    _ => println!("nodel down")
  }

  assert!(channel.wait_close().is_ok());
}