

mod auth;

use clap::{Arg, App};
use crate::auth::run;

fn main() {
    let matches = App::new("agfun")
        .version("0.1.0")
        .author("chenfeng <979924272@qq.com>")
        .about("agfun command-line")
        .arg(Arg::with_name("APP")
            .help("server to run")
            .empty_values(false)
        )
        .get_matches();

        match matches.value_of("APP").unwrap() {
            "auth" => {
                run()
            }
            _ => unreachable!(),
        }
}
