use std::path::Path;

use clap::{App, Arg};
use spinners::{Spinner, Spinners};
use must::tape::tape;

pub fn main() {
    let matches = App::new("must")
    .about("Magnetic tape interface in Rust")
    .arg(
        Arg::new("COMMAND")
        .about("Which command to execute")
        .possible_values(&[
            "erase",
            "fastforward-filemark",
            "fastforward-record",
            "rewind-filemark",
            "rewind-record",
            "status",
        ])
        .takes_value(true)
    )
    .arg(
        Arg::new("device")
        .about("Defines the tape device to operate on")
        .takes_value(true)
        .short('d')
        .long("device")
    )
    .arg(
        Arg::new("count")
        .about("Determines how many of COMMAND to run")
        .takes_value(true)
        .short('c')
        .long("count")
        .required_if_eq_any(&[
            ("COMMAND", "fastforward-record"),
            ("COMMAND", "fastforward-filemark"),
            ("COMMAND", "rewind-record"),
            ("COMMAND", "rewind-filemark"),
        ])
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

    let count: i32 = matches.value_of_t("count").unwrap_or(1);

    match matches.value_of("COMMAND").unwrap() {
        "status" => {
            tape::status(device)
        },
        "fastforward-record" | "fastforward-filemark" | "rewind-record" | "rewind-filemark" => {
            do_unary_tape_command(device, matches.value_of("COMMAND").unwrap(), count)
        },
        _ => unreachable!(),
    };
}

fn validate_device(device: &str) -> bool {
    Path::new(device).exists()
}

fn do_unary_tape_command(device: &str, command: &str, count: i32) -> i32 {
    let sp = Spinner::new(Spinners::Line, "Executing tape command".into());
    let (move_type, direction) = match command {
        "fastforward-record" => (tape::MovementType::Record, tape::MovementDirection::Forward),
        "fastforward-filemark" => (tape::MovementType::FileMark, tape::MovementDirection::Forward),
        "rewind-record" => (tape::MovementType::Record, tape::MovementDirection::Backward),
        "rewind-filemark" => (tape::MovementType::FileMark, tape::MovementDirection::Backward),
        _ => unreachable!(),
    };
    let res = tape::move_space(device, move_type, direction, count);
    sp.stop();
    println!();
    res
}
