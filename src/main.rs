use anyhow::{Context, Result};
use clap::{App, Arg};
use edid_gen::{CvtModeBuilder, Version};

fn main() -> Result<()> {
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

    let timing_name = matches
        .value_of("timing_name")
        .context("timing_name invalid")?;

    let output = matches.value_of("output").context("output invalid")?;

    let version = matches
        .value_of("edid_version")
        .context("edid version invalid")
        .map(|s| s.parse::<Version>())??;

    let reduced = matches.is_present("reduced");

    let x = matches
        .value_of("X")
        .context("param X invalid")
        .map(|s| s.parse::<i32>())??;

    let y = matches
        .value_of("Y")
        .context("param Y invalid")
        .map(|s| s.parse::<i32>())??;

    let refresh = matches
        .value_of("refresh")
        .context("param refresh invalid")
        .map(|s| s.parse::<i32>())??;

    let mode = CvtModeBuilder::new()
        .hdisplay(x)
        .vdisplay(y)
        .vrefresh(refresh)
        .reduced(reduced)
        .build();

    edid_gen::generate_edid(&mode, version, timing_name, output)
}
