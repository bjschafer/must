use std::path::Path;

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
            "rewind-record",
            "rewind-filemark",
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
    if ! validate_device(device) {
        println!("Invalid tape device {}", device);
        std::process::exit(66);
    }

    match matches.value_of("COMMAND").unwrap() {
        "status" => {
            tape::status(device)
        },
        "fastforward-record" | "fastforward-filemark" | "rewind-record" | "rewind-filemark" => {
            do_long_tape_command(device, matches.value_of("COMMAND").unwrap())
        },
        _ => unreachable!(),
    };
}

fn validate_device(device: &str) -> bool {
    Path::new(device).exists()
}

fn do_long_tape_command(device: &str, command: &str) -> i32 {
    let sp = Spinner::new(Spinners::Line, "Executing tape command".into());
    let (move_type, direction) = match command {
        "fastforward-record" => (tape::MovementType::Record, tape::MovementDirection::Forward),
        "fastforward-filemark" => (tape::MovementType::FileMark, tape::MovementDirection::Forward),
        "rewind-record" => (tape::MovementType::Record, tape::MovementDirection::Backward),
        "rewind-filemark" => (tape::MovementType::FileMark, tape::MovementDirection::Backward),
        _ => unreachable!(),
    };
    let res = tape::move_space(device, move_type, direction, 1);
    sp.stop();
    println!();
    res
}