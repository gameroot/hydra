extern crate ssh2;
extern crate serde_json;
use self::ssh2::Session;
use std::net::TcpStream;
use std::thread;
use std::io::Read;

use inventory;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileResult {
    status: String,
    message: String
}

 
fn push_file(host: &str, orgin: &str, destination: &str){
  let tcp = TcpStream::connect(host).unwrap();
  let mut sess = Session::new().unwrap();
  sess.handshake(&tcp).unwrap();
  sess.userauth_agent("jerome").unwrap();
  let mut channel = sess.channel_session().unwrap();
  
  //channel.exec(".hydra/agent").unwrap();
  
  let mut s = String::new();
  channel.read_to_string(&mut s).unwrap();
  let result: FileResult = serde_json::from_str(&s).unwrap();

  println!("{:?}", result);

  assert!(channel.wait_close().is_ok());
}