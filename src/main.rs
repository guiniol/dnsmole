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
                    .arg(Arg::with_name("port")
                         .help("port and/or interface to listen on (default 53)")
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
                    .arg(Arg::with_name("port")
                         .help("port and/or interface to listen on (default 5353)")
                         .short("p")
                         .long("port")
                         .takes_value(true)
                        )
                    .arg(Arg::with_name("relay")
                         .help("hostname[:port] of the dnsmole relay")
                         .index(1)
                         .required(true)
                         )
                    )
        .get_matches();

    match matches.subcommand_name() {
        Some("serve")   => println!("SERVE - target: {}, listen on {}", matches.value_of("target").unwrap(), matches.value_of("port").unwrap_or("*:53")),
        Some("connect") => println!("CONNECT - relay: {}, listen on {}", matches.value_of("relay").unwrap(), matches.value_of("port").unwrap_or("127.0.0.1:5353")),
        None            => println!("You must specify an action"),
        _               => println!("Unknown action"),
    }
}

