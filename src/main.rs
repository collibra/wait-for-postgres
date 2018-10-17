extern crate clap;
extern crate postgres;

use clap::{App, Arg};
use postgres::params::{Builder, Host};
use postgres::{Connection, TlsMode};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::Duration;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = App::new("wait-for-postgres")
        .version(VERSION)
        .author(AUTHORS)
        .about("Wait for a PostgreSQL server to be ready")
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .takes_value(true)
            .required(true)
            .help("Specifies the host name of the machine on which the server is running. If the value begins with a slash, it is used as the directory for the Unix-domain socket."))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .help("Specifies the TCP port or the local Unix-domain socket file extension on which the server is listening for connections. Defaults to 5432."))
        .arg(Arg::with_name("interval")
            .short("i")
            .long("interval")
            .takes_value(true)
            .help("Specifies the interval in milliseconds between each call. Defaults to 3000."))
        .arg(Arg::with_name("command")
            .multiple(true)
            .help("Command and arguments to execute (using execvp) at the end"))
        .get_matches();

    let host = matches
        .value_of("host")
        .map(|x| {
            if x.starts_with("/") {
                Host::Unix(x.into())
            } else {
                Host::Tcp(x.into())
            }
        }).unwrap();
    let port = matches
        .value_of("port")
        .map(|x| x.parse::<u16>().expect("could not parse numeric value"))
        .unwrap_or(5432);
    let interval = Duration::from_millis(
        matches
            .value_of("interval")
            .map(|x| x.parse::<u64>().expect("could not parse numeric value"))
            .unwrap_or(3000),
    );

    let params = Builder::new()
        // NOTE: we need to provide user AND password to avoid connection params error
        .user("postgres", Some("postgres"))
        .port(port)
        .build(host);
    loop {
        let conn = Connection::connect(params.clone(), TlsMode::None);
        match conn {
            Ok(_) => break,
            Err(ref e) if e.as_db().is_some() => break,
            Err(ref e) => eprintln!("{}", e),
        }

        sleep(interval);
    }

    if let Some(mut args) = matches.values_of("command") {
        let err = Command::new(args.next().unwrap()).args(args).exec();
        eprintln!("Could not execvp: {}", err);
        exit(1);
    }
}
