use clap::{Arg, App, SubCommand};
use std::panic;
use std::str;

mod protocol;

fn main() {
    // Set custom panic hook
    if !cfg!(debug_assertions) {
        panic::set_hook(Box::new(|hook| {
            if let Some(p) = hook.payload().downcast_ref::<&str>() {
                println!("FATAL ERROR: {}", p);
            }
        }));
    }

    let mut app = App::new("yant")
                        .version("1.0")
                        .author("penumbra23 <glbranimir@gmail.com>")
                        .about("Send requests over modern day network protocols")
                        .subcommand(SubCommand::with_name("http")
                            .about("Sends a HTTP request")
                                .arg(Arg::with_name("method")
                                    .short("m")
                                    .long("method")
                                    .default_value("GET")
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
                                    .default_value("")
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
                                    .default_value("")
                                    .help("Data inside the TCP payload"))
                                .arg(Arg::with_name("wait")
                                    .short("w")
                                    .long("wait")
                                    .help("Waits for the server to respond and prints the message")))
                        .subcommand(SubCommand::with_name("udp")
                            .about("Sends UDP packets to the specified target")
                                .arg(Arg::with_name("target")
                                    .short("t")
                                    .long("target")
                                    .takes_value(true)
                                    .required(true)
                                    .help("Target IP address & port of the UDP server socket (i.e. localhost:7788)"))
                                .arg(Arg::with_name("data")
                                    .short("d")
                                    .long("data")
                                    .takes_value(true)
                                    .default_value("")
                                    .help("Data inside the UDP payload"))
                                .arg(Arg::with_name("wait")
                                    .short("w")
                                    .long("wait")
                                    .help("Waits for the server to respond and prints the message")))
                        .subcommand(SubCommand::with_name("icmp")
                            .about("Pings the target machine using ICMP")
                                .arg(Arg::with_name("target")
                                    .short("t")
                                    .long("target")
                                    .takes_value(true)
                                    .required(true)
                                    .help("Target IP address & port of the host (i.e. localhost:7788)"))
                                .arg(Arg::with_name("count")
                                    .short("n")
                                    .long("count")
                                    .takes_value(true)
                                    .default_value("4")
                                    .help("Number of ICMP requests to make (default: 4)"))
                                .arg(Arg::with_name("data")
                                    .short("d")
                                    .long("data")
                                    .takes_value(true)
                                    .help("Send buffer payload data")))
                        .subcommand(SubCommand::with_name("jsonrpc")
                            .about("Invokes a JSON RPC 2.0 method on a remote server")
                                .arg(Arg::with_name("target")
                                    .short("t")
                                    .long("target")
                                    .takes_value(true)
                                    .required(true)
                                    .help("Target IP address & port of the host (i.e. localhost:7788)"))
                                .arg(Arg::with_name("method")
                                    .short("m")
                                    .long("method")
                                    .takes_value(true)
                                    .required(true)
                                    .help("RPC method on the host machine"))
                                .arg(Arg::with_name("params")
                                    .short("p")
                                    .long("params")
                                    .takes_value(true)
                                    .help("RPC method parameters"))
                                .arg(Arg::with_name("headers")
                                    .short("h")
                                    .long("headers")
                                    .takes_value(true)
                                    .multiple(true)
                                    .help("HTTP headers (syntax header:value, i.e. Authentication:Bearer)"))
                                .arg(Arg::with_name("version")
                                    .short("v")
                                    .long("ver")
                                    .takes_value(true)
                                    .default_value("2.0")
                                    .help("JSON RPC version (default: 2.0)"))
                                .arg(Arg::with_name("id")
                                    .short("id")
                                    .long("id")
                                    .takes_value(true)
                                    .help("Unique request identifier (default: random GUID)")));

    let mut help = Vec::new();
    app.write_long_help(&mut help).expect("Failed to get help!");

    let matches = app.get_matches();

    // Match the command
    match matches.subcommand() {
        ("http", Some(matches)) => protocol::http::handle_http(&matches),
        ("tcp", Some(matches)) => protocol::tcp::handle_tcp(&matches),
        ("udp", Some(matches)) => protocol::udp::handle_udp(&matches),
        ("icmp", Some(matches)) => protocol::icmp::handle_icmp(&matches),
        ("jsonrpc", Some(matches)) => protocol::jsonrpc::handle_jsonrpc(&matches),
        _ => { println!("{}", str::from_utf8(&help[..help.len()]).unwrap()); () },
    }
}