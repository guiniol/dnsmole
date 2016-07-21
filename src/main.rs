#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};


fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("serve")
                    .about("Create a server to relay remote connections")
                    .arg(Arg::with_name("socket")
                         .help("port and/or interface to listen on")
                         .short("p")
                         .long("port")
                         .takes_value(true)
                         )
                    .arg(Arg::with_name("target")
                         .help("[ip|hostname:]port to redirect to")
                         .index(1)
                         .required(true)
                         )
                    )
        .subcommand(SubCommand::with_name("connect")
                    .about("Connect to a dnsmole server")
                    .arg(Arg::with_name("socket")
                         .help("port and/or interface to listen on")
                         .short("p")
                         .long("port")
                         .takes_value(true)
                         )
                    .arg(Arg::with_name("target")
                         .help("hostname[:port] of the dnsmole server")
                         .index(1)
                         .required(true)
                         )
                    )
        .get_matches();

    match matches.subcommand_name() {
        Some("serve")   => println!("SERVE - target: {}, listen on {}", matches.value_of("target").unwrap(), matches.value_of("socket").unwrap_or("*:25")),
        Some("connect") => println!("CONNECT - target: {}, listen on {}", matches.value_of("target").unwrap(), matches.value_of("socket").unwrap_or("127.0.0.1:2525")),
        None            => println!("You must specify an action"),
        _               => println!("Unknown action"),
    }
}

