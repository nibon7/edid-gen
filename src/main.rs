use anyhow::Result;
use edid_gen::{CvtModeBuilder, Version};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "edid-gen", version = "0.1", about = "Generate EDID file")]
struct Opt {
    /// Timing name
    #[structopt(short, long)]
    timing_name: String,
    /// Ouput file
    #[structopt(short, long)]
    output: String,
    /// EDID version (v1.0 to v1.4)
    #[structopt(short, long, default_value = "1.4")]
    version: Version,
    /// Create a mode with reduced blanking
    #[structopt(short, long)]
    reduced: bool,
    /// Desired horizontal resolution (multiple of 8, required)
    x: i32,
    /// Desired vertical resolution (required)
    y: i32,
    /// Desired refresh rate
    #[structopt(default_value = "60")]
    refresh: i32,
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    let mode = CvtModeBuilder::new()
        .hdisplay(opt.x)
        .vdisplay(opt.y)
        .vrefresh(opt.refresh)
        .reduced(opt.reduced)
        .build();

    edid_gen::generate_edid(&mode, opt.version, &opt.timing_name, &opt.output)
}
