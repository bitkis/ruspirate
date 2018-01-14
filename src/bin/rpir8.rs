#[macro_use]
extern crate clap;

extern crate ruspirate;

use ruspirate::{Devices};
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {

    let matches = clap_app!(MyApp =>
                            (@setting SubcommandRequiredElseHelp)
                            (version: VERSION)
                            (author: "Geoff Cant <geoff+rust@archant.us>")
                            (about: "Bus pirates things. With Rust!")
                            (@subcommand list =>
                             (about: "List buspirates"))
                            (@subcommand test =>
                             (about: "Test a buspirate")
                             (@arg dev: -d --dev +takes_value
                              "The bus pirate device to use."))
    ).get_matches();

    let pirates = Devices::detect();

    match matches.subcommand_name() {
        Some("list") => {
            match pirates.len() {
                0 => {
                    println!("No bus pirates found.");
                    std::process::exit(1);
                }
                _ => {
                    for (i, p) in pirates.into_iter().enumerate() {
                        println!("({}) {dev:?} ({hwid})",
                                 i+1, dev=p.device, hwid=p.hwid);
                    }
                }
            }
        },
        Some("test") => {
            let test = matches.subcommand_matches("test").unwrap();

            let device = match test.value_of("dev") {
                None => pirates.default(),
                Some(pat) => pirates.find(pat)
            };

            match device {
                None => {
                    println!("No bus pirate found.");
                    std::process::exit(1);
                },
                Some(pirate) => {
                    println!("Testing {:?}", pirate);
                    match pirate.open() {
                        Ok(_port) => {
                            println!("Yay! Opened {:?}", pirate.device.to_str())
                        },
                        Err(e) => {
                            println!("Couldn't open {:?}: {}",
                                     pirate.device.to_str(), e);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
        _ => {
            println!("Unknown subcommand.");
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}