use std::io::Read;
use std::fs::File;
use std::env;

use bendy::decoding::{Error, FromBencode};
mod bencode;
mod tracker;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;
    
    let torrent = bencode::MetaInfo::from_bencode(&bytes)?;
    let url = tracker::get_url(&torrent.announce);
    println!("{:#?}", &torrent);
    println!("{:#?}", &url);

    Ok(())
}
