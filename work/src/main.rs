
pub mod movie;
pub mod auth;

use movie::{movie};
use clap::{Arg};

fn main() {
    movie();
    println!("Hello, world!");
}
