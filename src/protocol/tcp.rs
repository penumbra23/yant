use clap:: ArgMatches;
use std::net::TcpStream;
use std::io::prelude::*;
use std::str;

pub fn handle_tcp(matches: &ArgMatches) {
    let server = matches.value_of("target").unwrap();
    let data = String::from(matches.value_of("data").unwrap());

    let mut stream = TcpStream::connect(server).unwrap();

    stream.write_all(&data.as_bytes()).unwrap();

    if matches.is_present("wait") {
        let mut buff = vec![0; 4096];
        let received = stream.read(&mut buff).unwrap();
        println!("{:?}", str::from_utf8(&buff[..received]).unwrap());
    }
}