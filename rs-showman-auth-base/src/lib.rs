pub mod session;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref SIGN_KEY: Vec<u8> = {
        let mut key = Vec::new();

        let mut file = File::open("pass.key").unwrap();
        file.read(&mut key).unwrap();

        key
    };
}