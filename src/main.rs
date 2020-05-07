extern crate clap;

use bioseg::utils::*;

use clap::{App, Arg};

fn main() {
    let matches = App::new("bio-segmenter")
        .version("0.1")
        .author("William Gasper <wigasper@gmail.com>")
        .arg(
            Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .required(true)
                .takes_value(true)
                .help("input file path"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .default_value("output.png")
                .help("output file path"),
        )
        .arg(
            Arg::with_name("RADIUS")
                .short("r")
                .long("radius")
                .default_value("3")
                .help("initial radius for neighbor checking"),
        )
        .arg(
            Arg::with_name("RADIUS_MULTIPLIER")
                .short("m")
                .long("multiplier")
                .default_value("3.0")
                .help("multiplies the radius by this value for each new round of abstraction"),
        )

        .arg(
            Arg::with_name("THRESHOLD")
                .short("t")
                .long("threshold")
                .default_value("5")
                .help("maximum delta allowed to add an edge between nodes"),
        )
        .get_matches();

    let input_fp = matches.value_of("INPUT").unwrap();
    let output_fp = matches.value_of("OUTPUT").unwrap();

    let radius: u32 = matches
        .value_of("RADIUS")
        .unwrap()
        .parse()
        .unwrap_or_else(|why| {
            panic!(
                "Could not parse '{}' to u32: {}",
                matches.value_of("RADIUS").unwrap(),
                why
            );
        });

    let radius_multiplier: f32 = matches
        .value_of("RADIUS_MULTIPLIER")
        .unwrap()
        .parse()
        .unwrap_or_else(|why| {
            panic!(
                "Could not parse '{}' to f32: {}",
                matches.value_of("RADIUS_MULTIPLIER").unwrap(),
                why
            );
        });

    let threshold: u8 = matches
        .value_of("THRESHOLD")
        .unwrap()
        .parse()
        .unwrap_or_else(|why| {
            panic!(
                "Could not parse '{}' to u8: {}",
                matches.value_of("THRESHOLD").unwrap(),
                why
            );
        });

    segment_image(input_fp, output_fp, radius, threshold, radius_multiplier);
}
