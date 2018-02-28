extern crate ssh2;

pub mod profiler;
pub mod pinger;

pub fn profile_hosts(){
  profiler::profile_remote_hosts();
}

pub fn ping_remote_hosts(){
  pinger::ping_remote_hosts();
}
