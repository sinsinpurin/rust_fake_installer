extern crate fake_installer;

use std::{process};

use fake_installer::{Installer, Mode};
fn main() {
    let f_installer = Installer::new(3, Mode::InstallerFile).unwrap_or_else(|err|{
            eprintln!("Problem!! : {}", err);
            process::exit(1);
        }
    );
    f_installer.run();
    println!("Hello, world!");
}
