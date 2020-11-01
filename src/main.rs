use std::path::Path;

use clap::{App, Arg};
use spinners::{Spinner, Spinners};
use must::tape::tape;
use must::backup::backup;

pub fn main() {
    let matches = App::new("must")
    .about("Magnetic tape interface and backup in Rust")
    .arg(
        Arg::new("device")
        .about("Defines the tape device to operate on")
        .takes_value(true)
        .short('d')
        .long("device")
    )
    .subcommand(
        App::new("backup")
        .about("Backup to tape")
        .arg(
            Arg::new("paths")
            .about("Path(s) to backup")
            .short('p')
            .long("paths")
            .takes_value(true)
            .multiple(true)
            .required(true)
        )
        .arg(
            Arg::new("compression")
            .about("Compression method")
            .short('z')
            .long("compression")
            .possible_values(&[
                "none",
                "bzip2",
                "gzip",
                "lz4",
                "zstd",
            ])
            .takes_value(true)
            .default_value("none")
        )
        .arg(
            Arg::new("encryption-enable")
            .about("Encrypt backups?")
            .short('e')
            .long("encrypt")
            .requires("encryption-keyfile")
        )
        .arg(
            Arg::new("encryption-keyfile")
            .about("Encryption keyfile")
            .short('k')
            .long("keyfile")
            .takes_value(true)
        )
    )
    .subcommand(
        App::new("tape")
        .about("Interfaces with the tape drive")
        .alias("tape-control")
        .arg(
            Arg::new("COMMAND")
            .about("Which command to execute")
            .possible_values(&[
                "erase",
                "rewind",
                "status",
                "forward-filemark",
                "forward-record",
                "back-filemark",
                "back-record",
            ])
            .takes_value(true)
        )
        .arg(
            Arg::new("count")
            .about("Determines how many of COMMAND to run (default 1)")
            .takes_value(true)
            .short('c')
            .long("count")
            .required_if_eq_any(&[
                ("COMMAND", "forward-record"),
                ("COMMAND", "forward-filemark"),
                ("COMMAND", "back-record"),
                ("COMMAND", "back-filemark"),
            ])
        )
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
        "rewind" => {
            do_rewind(device)
        }
        "forward-record" | "forward-filemark" | "back-record" | "back-filemark" => {
            do_unary_tape_command(device, matches.value_of("COMMAND").unwrap(), count)
        },
        _ => unreachable!(),
    };
}

fn validate_device(device: &str) -> bool {
    Path::new(device).exists()
}

fn do_rewind(device: &str) -> i32 {
    let sp = Spinner::new(Spinners::Line, "Executing tape command".into());
    let res = tape::rewind(device);
    sp.stop();
    println!();
    res
}

fn do_unary_tape_command(device: &str, command: &str, count: i32) -> i32 {
    let sp = Spinner::new(Spinners::Line, "Executing tape command".into());
    let (move_type, direction) = match command {
        "forward-record" => (tape::MovementType::Record, tape::MovementDirection::Forward),
        "forward-filemark" => (tape::MovementType::FileMark, tape::MovementDirection::Forward),
        "back-record" => (tape::MovementType::Record, tape::MovementDirection::Backward),
        "back-filemark" => (tape::MovementType::FileMark, tape::MovementDirection::Backward),
        _ => unreachable!(),
    };
    let res = tape::move_space(device, move_type, direction, count);
    sp.stop();
    println!();
    res
}
