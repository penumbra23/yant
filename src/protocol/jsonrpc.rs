use clap::ArgMatches;
use curl::easy::{Easy, List};
use serde_json::Value;
use serde::{Serialize, Deserialize}; 
use guid_create::GUID;
use std::str;

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    #[serde(skip_serializing_if = "is_empty")]
    params: Option<Value>,
    id: String
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "is_empty")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "is_empty")]
    error: Option<Value>,
    id: String
}

fn is_empty(s: &Option<Value>) -> bool {
    s.is_none()
}

pub fn handle_jsonrpc(matches: &ArgMatches) {
    let server = matches.value_of("target").unwrap();
    let method = matches.value_of("method").unwrap();
    let params: Option<Value> = matches.value_of("params").map(|s| serde_json::from_str(&s).unwrap());
    let version = matches.value_of("version").unwrap_or("2.0");
    let guid = GUID::rand().to_string();
    let id = matches.value_of("id").unwrap_or(guid.as_str());

    let mut headers = List::new();
    headers.append("Content-Type: application/json").unwrap();

    if let Some(header_vals) = matches.values_of("headers") {
        for x in header_vals.collect::<Vec<_>>() {
            headers.append(x).unwrap();
        }
    }

    let req = JsonRpcRequest {
        jsonrpc: String::from(version),
        method: String::from(method),
        params: params,
        id: String::from(id)
    };

    let mut easy = Easy::new();
    easy.url(server).unwrap();
    easy.custom_request("POST").unwrap();
    easy.http_headers(headers).unwrap();
    easy.post_fields_copy(serde_json::to_string(&req).unwrap().as_bytes()).unwrap();
    easy.write_function(|data| {
        let res: JsonRpcResponse = serde_json::from_str(str::from_utf8(&data[..data.len()]).unwrap()).unwrap();
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}