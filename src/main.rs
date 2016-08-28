#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

mod server;
mod client;


fn main() {
    let matches = App::with_defaults(env!("CARGO_PKG_NAME"))
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

    match matches.subcommand() {
        ("serve",   Some(submatch)) => server::serve(submatch.value_of("target").unwrap(), submatch.value_of("port").unwrap_or("*:53")),
        ("connect", Some(submatch)) => client::connect(submatch.value_of("relay").unwrap(), submatch.value_of("port").unwrap_or("127.0.0.1:5353")),
        _                           => println!("Error"),
    }
}

