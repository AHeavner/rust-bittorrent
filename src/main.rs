use std::io::Read;
use std::fs::File;
use std::env;

#[macro_use]
extern crate serde_derive;

use serde_bencode::de;
mod bencode;
mod tracker;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    
    let torrent: bencode::Torrent = de::from_bytes(&bytes).unwrap();
    bencode::render_torrent(&torrent);

    let url = tracker::get_url(&torrent.announce.unwrap());
    //println!("{:#?}", &torrent);
    println!("{:#?}", &url);
}
