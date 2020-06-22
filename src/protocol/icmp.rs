use clap::ArgMatches;
use std::str;

#[cfg(target_os = "linux")]
use fastping_rs::Pinger;
#[cfg(target_os = "linux")]
use fastping_rs::PingResult::{Idle, Receive};

#[cfg(target_os = "windows")]
use winping::{Buffer, Pinger};

#[cfg(target_os = "linux")]
pub fn handle_icmp(matches: &ArgMatches) {
    let server = matches.value_of("target").unwrap();
    let count = matches.value_of("count").unwrap().parse::<i32>().unwrap();

    let (pinger, results) = match Pinger::new(None, None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e)
    };

    pinger.add_ipaddr(server);
    
    println!("Pinging address {}, request number {}:", server, count);

    for _ in 0..count {
        pinger.ping_once();
        match results.recv() {
            Ok(result) => {
                match result {
                    Idle{addr} => { /*DO NOTHING*/ },
                    Receive{addr, rtt} => println!("Response time: {} ms.", rtt.as_millis() as u64),
                }
            },
            Err(e) => panic!("{}", e),
        }
    }
}

#[cfg(target_os = "windows")]
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