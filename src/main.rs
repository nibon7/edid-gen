use clap::{App, Arg};
use edid_gen::{CvtMode, Version};

fn handle_error_and_exit(msg: &str) -> ! {
    eprintln!("parameter '{}' error", msg);
    std::process::exit(1);
}

fn main() -> anyhow::Result<()> {
    let matches = App::new("edid-gen")
        .about("Generate EDID file")
        .version("0.1")
        .arg(
            Arg::with_name("timing_name")
                .long("timing-name")
                .short("t")
                .takes_value(true)
                .help("Timing name")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .takes_value(true)
                .help("Output file")
                .required(true),
        )
        .arg(
            Arg::with_name("edid_version")
                .long("edid-version")
                .short("v")
                .takes_value(true)
                .possible_values(&["1.0", "1.1", "1.2", "1.3", "1.4"])
                .help("EDID version (v1.0 to v1.4)")
                .default_value("1.4"),
        )
        .arg(
            Arg::with_name("reduced")
                .long("reduced")
                .short("r")
                .help("Create a mode with reduced blanking"),
        )
        .arg(
            Arg::with_name("X")
                .required(true)
                .help("Desired horizontal resolution (multiple of 8, required)"),
        )
        .arg(
            Arg::with_name("Y")
                .required(true)
                .help("Desired vertical resolution (required)"),
        )
        .arg(
            Arg::with_name("refresh")
                .help("Desired refresh rate")
                .default_value("60"),
        )
        .get_matches();

    let timing_name = match matches.value_of("timing_name") {
        Some(s) => s,
        _ => handle_error_and_exit("timing_name"),
    };

    let output = match matches.value_of("output") {
        Some(s) => s,
        _ => handle_error_and_exit("output"),
    };

    let version = match matches.value_of("edid_version") {
        Some(s) => Version::from(s),
        _ => handle_error_and_exit("edid_version"),
    };

    let reduced = matches.is_present("reduced");

    let x = match matches.value_of("X") {
        Some(s) => s.parse::<i32>()?,
        _ => handle_error_and_exit("X"),
    };

    let y = match matches.value_of("Y") {
        Some(s) => s.parse::<i32>()?,
        _ => handle_error_and_exit("Y"),
    };

    let refresh = match matches.value_of("refresh") {
        Some(s) => s.parse::<i32>()?,
        _ => handle_error_and_exit("refresh"),
    };

    let mode = CvtMode::new(x, y, refresh, reduced, false, false);

    edid_gen::generate_edid(&mode, version, timing_name, output)?;

    Ok(())
}
