#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgGroup};


fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("serve")
             .help("Create a server to relay remote connections")
             .short("s")
             .long("serve")
             )
        .arg(Arg::with_name("connect")
             .help("Connect to a dnsmole server")
             .short("c")
             .long("connect")
             )
        .group(ArgGroup::with_name("action")
               .required(true)
               .args(&["serve", "connect"])
               )
        .arg(Arg::with_name("port")
             .help("port and/or interface to listen on (default 53)")
             .short("p")
             .long("port")
             .takes_value(true)
             .requires("serve")
            )
        .arg(Arg::with_name("target")
             .help("[ip|hostname:]port to redirect to or dnsmole server to connect to")
             .index(1)
             .required(true)
            )
        .arg(Arg::with_name("socket")
             .help("port and/or interface to listen on (default 5353)")
             .short("k")
             .long("socket")
             .takes_value(true)
             .requires("connect")
            )
        .get_matches();

    if matches.is_present("serve") {
        println!("SERVE - target: {}, listen on {}", matches.value_of("target").unwrap(), matches.value_of("port").unwrap_or("*:53"));
    }
    if matches.is_present("connect") {
        println!("CONNECT - relay: {}, listen on {}", matches.value_of("target").unwrap(), matches.value_of("socket").unwrap_or("127.0.0.1:5353"));
    }
}

