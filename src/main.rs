use clap::{App, Arg};

    //Defines the tape device to operate on.
    // #[clap(short, long, default_value = "/dev/nst0")]
    // device: String,
    //Verbosity. More means louder.
    // #[clap(short, long, parse(from_occurrences))]
    // verbose: i32,

use must::tape::tape;
pub fn main() {
    let matches = App::new("must")
    .about("Magnetic tape interface in Rust")
    .arg(
        Arg::new("device")
        .about("Defines the tape device to operate on")
        .takes_value(true)
        .short('d')
        .long("device")
    )
    .arg(
        Arg::new("COMMAND")
        .about("Which command to execute")
        .index(1)
        .possible_values(&[
            "status",
            "fastforward",
            "rewind",
        ])
        .required(true),
    )
    .get_matches();

    let device: &str;

    if let Some(dev) = matches.value_of("device") {
        device = dev;
    }
    else {
        device = "/dev/nst0";
    }

    match matches.value_of("COMMAND").unwrap() {
        "status" => tape::status(device),
        "fastforward" => tape::fastforward(device, 1),
        "rewind" => tape::rewind(device),
        _ => unreachable!(),
    };
}