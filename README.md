# YANT
Yet Another Network Tool

## Intro
**YANT** is a nice and easy CLI tool for making network requests over various modern day protocols. It's written in the awesome Rust language and has binaries for Windows & Linux.

## Usage
```
USAGE:
    yant [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    	Prints this message or the help of the given subcommand(s)
    http    	Sends a HTTP request
    icmp    	Sends UDP packets to the specified target
    tcp     	Connects and sends a message over TCP
    udp     	Sends UDP packets to the specified target
    jsonrpc	Invokes a JSON RPC method on a remote server

EXAMPLES:
    yant http --target https://ipinfo.io/json
    yant icmp --target 8.8.8.8
    yant tcp --target 45.22.12.23:6777 --data "hello"
```
## Contribute

Feel free to open issues, recommend new features, test and open pull requests on this repo. YANT is definitely going to grow and expand!

## License
MIT
