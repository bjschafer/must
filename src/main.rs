use clap::{App, Arg};
use spinners::{Spinner, Spinners};
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
            "fastforward-record",
            "fastforward-filemark",
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
        "status" => {
            tape::status(device)
        },
        "fastforward-record" | "fastforward-filemark" | "rewind" => do_long_tape_command(device, matches.value_of("COMMAND").unwrap()),
        _ => unreachable!(),
    };
}

fn do_long_tape_command(device: &str, command: &str) -> i32 {
    let sp = Spinner::new(Spinners::Line, "Executing tape command".into());
    let res = match command {
        "fastforward-record" => tape::fastforward_record(device, 1),
        "fastforward-filemark" => tape::fastforward_filemark(device, 1),
        "rewind" => tape::rewind(device),
        _ => -1,
    };
    sp.stop();
    println!();
    res
}