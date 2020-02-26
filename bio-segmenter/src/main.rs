mod lib;
pub use crate::lib::graphbuilder::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fp = &args[1];
    let radius: u32 = args[2].parse().unwrap_or_else(|why| {
        panic!("msg here");
    });

    let threshold: u64 = args[3].parse().unwrap_or_else(|why| {
        panic!("msg here");
    });

    let adj_list = build_graph(fp.to_owned(), radius, threshold);

}
