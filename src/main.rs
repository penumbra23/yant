use std::io::{stdout, Write};
use clap::{Arg, App, SubCommand};
use curl::easy::{Easy, List};
use std::panic;

use std::net::TcpStream;
use std::io::prelude::*;

fn validate_http_method(method: &String) {
    match method.as_str() {
        "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "OPTIONS" | "HEAD" => (),
        _ => panic!("Not a valid HTTP method!"),
    }
}

fn main() {
    // Set custom panic hook
    if !cfg!(debug_assertions) {
        panic::set_hook(Box::new(|hook| {
            if let Some(p) = hook.payload().downcast_ref::<&str>() {
                println!("FATAL ERROR: {}", p);
            }
        }));
    }
    
    let matches = App::new("yant")
                        .version("1.0")
                        .author("penumbra23 <glbranimir@gmail.com>")
                        .about("Send requests over modern day network protocols")
                        .subcommand(SubCommand::with_name("http")
                            .about("Sends a HTTP request")
                                .arg(Arg::with_name("method")
                                    .short("m")
                                    .long("method")
                                    .takes_value(true)
                                    .help("HTTP method (possible values: GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS)"))
                                .arg(Arg::with_name("target")
                                    .short("t")
                                    .long("target")
                                    .takes_value(true)
                                    .required(true)
                                    .help("Target IP address of the request"))
                                .arg(Arg::with_name("headers")
                                    .short("h")
                                    .long("headers")
                                    .takes_value(true)
                                    .multiple(true)
                                    .help("HTTP headers (syntax header:value, i.e. Authentication:Bearer)"))
                                .arg(Arg::with_name("body")
                                    .short("b")
                                    .long("body")
                                    .takes_value(true)
                                    .help("Content of the request body")))
                        .subcommand(SubCommand::with_name("tcp")
                            .about("Connects and sends a message over TCP")
                                .arg(Arg::with_name("target")
                                    .short("t")
                                    .long("target")
                                    .takes_value(true)
                                    .required(true)
                                    .help("Target IP address & port of the TCP server socket (i.e. localhost:7788)"))
                                .arg(Arg::with_name("data")
                                .short("d")
                                .long("data")
                                .takes_value(true)
                                .required(true)
                                .help("Data inside the TCP payload")))
                        .get_matches();

    if let Some(matches) = matches.subcommand_matches("http") {
        let mut method = String::from("GET");
        let server = matches.value_of("target").unwrap();
        let payload = String::from(matches.value_of("body").unwrap());

        let mut headers = List::new();

        if let Some(header_vals) = matches.values_of("headers") {
            for x in header_vals.collect::<Vec<_>>() {
                headers.append(x).unwrap();
            }
        }

        if matches.is_present("method") {
            method = String::from(matches.value_of("method").unwrap().to_uppercase());
        }

        validate_http_method(&method);

        let mut easy = Easy::new();
        easy.url(server).unwrap();
        easy.custom_request(&method.to_uppercase()).unwrap();
        easy.http_headers(headers).unwrap();
        easy.post_fields_copy(payload.as_bytes()).unwrap();
        easy.write_function(|data| {
            stdout().write_all(data).unwrap();
            Ok(data.len())
        }).unwrap();
        easy.perform().unwrap();
    } else if let Some(matches) = matches.subcommand_matches("tcp") {
        let server = matches.value_of("target").unwrap();
        let data = String::from(matches.value_of("data").unwrap());

        let mut stream = TcpStream::connect(server).unwrap();

        stream.write(&data.as_bytes()).unwrap();
        let mut buff = vec![];
        stream.read_to_end(&mut buff).unwrap();
        println!("{:?}", buff);
        stdout().write_all(&buff).unwrap();
    }
}