extern crate fake_installer;

use std::env;
use std::{process};

use fake_installer::{Installer, Mode};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mode;
        match args[1].as_str(){
            "installfile" => {
                mode = Mode::InstallFile;
            }
            "hacking" => {
                mode = Mode::Hacking;
            }
            _ => {
                eprintln!("Error: Please input collect mode name.");
                process::exit(10);
            }
        }

        let f_installer = Installer::new(mode).unwrap_or_else(|err|{
                eprintln!("Problem!! : {}", err);
                process::exit(1);
            }
        );
        f_installer.run();
    }else{
        eprintln!("Error: Args isn't enough.");
        process::exit(10);
    }
    println!("Hello, world!");
}
