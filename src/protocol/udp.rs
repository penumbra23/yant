use clap:: ArgMatches;
use std::str;
use std::net::UdpSocket;

pub fn handle_udp(matches: &ArgMatches) {
    let server = matches.value_of("target").unwrap();
    let data = String::from(matches.value_of("data").unwrap());

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(data.as_bytes(), &server).unwrap();

    if matches.is_present("wait") {
        let mut buff = vec![0; 4096];
        match socket.recv(&mut buff) {
            Ok(received) => println!("{}", str::from_utf8(&buff[..received]).unwrap()),
            Err(e) => panic!("{}", e),
        }
    }
}