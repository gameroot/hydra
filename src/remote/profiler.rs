extern crate ssh2;
extern crate serde_json;
use self::ssh2::Session;
use std::net::TcpStream;
use std::thread;
use std::io::Read;

use inventory;

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemProfile {
    cpu_num: String,
    cpu_speed: String,
    hostname: String,
    os_type: String,
    os_release: String
}

pub fn profile_remote_hosts() {
  println!("ssh_ping_remote_hosts // inner");
  match inventory::get_hosts() {
    Err(_) => 
      println!("failed to get hosts"),
    Ok(hosts) =>
      for host in hosts {
        let handler = thread::spawn(move || {
            profile_host(&host);
        });
      },
  }
}

fn profile_host(host: &str){
  let tcp = TcpStream::connect(host).unwrap();
  let mut sess = Session::new().unwrap();
  sess.handshake(&tcp).unwrap();
  sess.userauth_agent("jerome").unwrap();
  let mut channel = sess.channel_session().unwrap();
  channel.exec(".hydra/agent").unwrap();
  let mut s = String::new();
  channel.read_to_string(&mut s).unwrap();
  let prof: SystemProfile = serde_json::from_str(&s).unwrap();

  // Do something with the profile
  println!("{:?}", prof);
  // we can match like this
  // match(s.contains("success")) {
  //   true => println!("node {} up", host),
  //   _ => println!("nodel down")
  // }
  assert!(channel.wait_close().is_ok());
}