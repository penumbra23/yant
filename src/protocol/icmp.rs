use clap::ArgMatches;
use std::str;

use winping::{Buffer, Pinger};

pub fn handle_icmp(matches: &ArgMatches) {
    let server = matches.value_of("target").unwrap();
    let count = matches.value_of("count").unwrap().parse::<i32>().unwrap();
    let data = if matches.is_present("data") {
        String::from(matches.value_of("data").unwrap())
    } else {
        String::from("")
    };
    
    let pinger = Pinger::new().unwrap();
    let mut buffer = Buffer::with_data(data.as_bytes().to_vec());
    
    println!("Pinging address {}, request number {}:", server, count);
    for _ in 0..count {
        match pinger.send(server.parse().unwrap(), &mut buffer) {
            Ok(rtt) => println!("Response time: {} ms. Response data: {}", rtt, str::from_utf8(buffer.reply_data()).unwrap()),
            Err(e) => panic!("{}", e),
        }
    }
}