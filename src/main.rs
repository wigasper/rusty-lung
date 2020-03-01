//pub use crate::lib::graphbuilder::*;
use bioseg::label_prop::*;
use bioseg::utils::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fp = &args[1];
    let radius: u32 = args[2].parse().unwrap_or_else(|why| {
        panic!("msg here");
    });

    let threshold: u8 = args[3].parse().unwrap_or_else(|why| {
        panic!("msg here");
    });

    segment_image(fp, radius, threshold);
    //let adj_list = build_adj_list(fp.to_owned(), radius, threshold);
}
