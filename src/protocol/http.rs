

use clap:: ArgMatches;
use curl::easy::{Easy, List};
use std::str;

pub fn handle_http(matches: &ArgMatches) {
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
        method = matches.value_of("method").unwrap().to_uppercase();
    }

    validate_http_method(&method);

    let mut easy = Easy::new();
    easy.url(server).unwrap();
    easy.custom_request(&method.to_uppercase()).unwrap();
    easy.http_headers(headers).unwrap();
    easy.post_fields_copy(payload.as_bytes()).unwrap();
    easy.write_function(|data| {
        println!("{}", str::from_utf8(&data[..data.len()]).unwrap());
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

fn validate_http_method(method: &str) {
    match method {
        "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "OPTIONS" | "HEAD" => (),
        _ => panic!("Not a valid HTTP method!"),
    }
}
