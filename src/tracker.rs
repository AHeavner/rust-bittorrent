use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use url::{Url};

pub fn get_url(url: &str) -> SocketAddr {
    let parsed_url = Url::parse(&url).unwrap();
    let host_port = format!("{}:{}", &parsed_url.host_str().unwrap(), &parsed_url.port().unwrap());
    let mut address_iter = host_port.to_socket_addrs().unwrap();
    //dbg!(&address_iter);
    let tracker_url = address_iter.next().unwrap();
    tracker_url
}

pub fn contact_tracker(url: &str) {
    
}

//get information from tracker
    //using udp library

