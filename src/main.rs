extern crate clap;
extern crate port_scanner;

use clap::App;
use clap::ArgMatches;
use clap::Arg;
use port_scanner::scan_port_addr;
use std::fs::File;
use std::net::TcpStream;
use std::process;
use std::io::Write;


fn main() {
    let matches = App::new("Rust Port Scanner")
        .version("0.1.0")
        .about("A simple port scanner")
        .arg(Arg::with_name("target")
            .short("t")
            .long("target")
            .help("Sets the target address")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Sets port to scan. If this option is not used, first 1000 ports will be scanned.")
            .takes_value(true))
        .arg(Arg::with_name("output")
                .short("O")
                .long("output")
                .help("Sets an output file")
                .takes_value(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("If -O is used, output will be showed on standard output too. Otherwise, nothing will change.")
            .takes_value(false))
        .get_matches();

    //set target address
    let target = matches.value_of("target").unwrap();

    //scan single port
    if matches.is_present("port") {
        let port: &str;
        port = matches.value_of("port").unwrap();
        output(&matches,format!("Port {} is {}", port, tcp_port_scan(target, port)));
        process::exit(0);
    }

    //scan first 1000 ports
    let port: i32;
    for port in 1..1000 {
        let is_open = scan_port_addr(format!("{}:{}", target, port));
        if is_open == true {
            output( &matches,format!("Port {} is open", port));
        }
    }
}


pub fn tcp_port_scan(target: &str, port: &str) -> String {
    if let Ok(stream) = TcpStream::connect(format!("{}:{}", target, port)) {
        String::from("open")
    } else {
        String::from("closed")
    }
}

fn output(matches: &ArgMatches, message: String) {
    if matches.is_present("output") {
        let filename = matches.value_of("output").unwrap();
        let mut out = File::create(filename).expect("Error: cannot create output file");
        writeln!(out, "{}", message).expect("Error: cannot write on output file");
        if matches.is_present("verbose") {
            println!("{}", message);
        }
    } else {
        println!("{}", message);
    }
}