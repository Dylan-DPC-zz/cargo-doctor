extern crate reqwest;
#[macro_use]
extern crate failure;
extern crate cargo_metadata;
extern crate clap;
extern crate indexmap;
extern crate scraper;

pub mod accessor;
pub mod handler;
pub mod linker;

use clap::{App, Arg};
use handler::Handler;

fn main() {
    let matches = App::new("Doctor")
        .version("0.1.0")
        .author("Dylan DPC <dylan.dpc@gmail.com")
        .about("Keeps your docs alive")
        .arg(
            Arg::with_name("manifest-path")
                .long("manifest-path")
                .value_name("PATH")
                .takes_value(true),
        ).arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("PATH")
                .help("Custom path of the docs")
                .takes_value(true),
        ).arg(
            Arg::with_name("remote")
                .short("r")
                .long("remote")
                .takes_value(false)
                .help("accesses documentation on docs.rs"),
        ).arg(
            Arg::with_name("local")
                .short("l")
                .long("local")
                .takes_value(false),
        ).help("accesses documentation stored locally")
        .get_matches();

    let input_path = matches.value_of("PATH");

    let path = match input_path {
        Some(x) => x.to_string(),
        None if matches.is_present("remote") => {
            accessor::remote_path().expect("cannot access remote path")
        }
        None if matches.is_present("local") => {
            accessor::local_path().expect("cannot access local path")
        }
        _ => panic!("path to docs cannot be found"),
    };

    let results = if matches.is_present("remote") {
        Handler::new(&path).remote_scrape()
    } else {
        Handler::new(&path).local_scrape()
    };

    match results {
        Ok(_) => println!("All links are healthy"),
        Err(error) => eprintln!("{}", error),
    };
}
