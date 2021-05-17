#[macro_use]
extern crate bitflags;

pub mod cvtmode;
pub use cvtmode::CvtMode;

use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

pub enum Version {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
    V1_4,
}

impl Version {
    pub fn major(&self) -> u8 {
        match *self {
            _ => 1,
        }
    }

    pub fn minor(&self) -> u8 {
        match *self {
            Self::V1_0 => 0,
            Self::V1_1 => 1,
            Self::V1_2 => 2,
            Self::V1_3 => 3,
            Self::V1_4 => 4,
        }
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "1.0" => Version::V1_0,
            "1.1" => Version::V1_1,
            "1.2" => Version::V1_2,
            "1.3" => Version::V1_3,
            "1.4" => Version::V1_4,
            _ => unreachable!(),
        }
    }
}

fn do_crc(data: &[u8]) -> Vec<u8> {
    let mut sum: u16 = 0;
    let mut v = data.to_owned();

    v.iter().for_each(|i| sum += *i as u16);

    v.pop();
    v.push((0x100 - sum % 0x100) as u8);

    v
}

pub fn generate_edid(
    mode: &CvtMode,
    version: Version,
    timing_name: &str,
    output: impl AsRef<Path>,
) -> std::io::Result<()> {
    let dir = tempdir()?.into_path();
    let dir = dir.as_path();

    let edid_temp_path = dir.join("edid.S.template");
    let edid_temp_asm = include_bytes!("edid.S.template");
    std::fs::write(&edid_temp_path, edid_temp_asm)?;

    let edid_path = dir.join("edid.S");
    let edid_asm = mode.generate_edid_asm(version, timing_name);
    std::fs::write(&edid_path, edid_asm.as_bytes())?;

    let edid_out = dir.join("edid.out");

    Command::new("cc")
        .arg("-c")
        .arg(&edid_path)
        .arg("-o")
        .arg(&edid_out)
        .current_dir(dir)
        .status()?;

    Command::new("objcopy")
        .arg("-O")
        .arg("binary")
        .arg("-j")
        .arg(".data")
        .arg(&edid_out)
        .arg(output.as_ref())
        .status()?;

    std::fs::remove_dir_all(dir)?;

    let data = std::fs::read(output.as_ref())?;
    let crc_data = do_crc(&data);
    std::fs::write(output.as_ref(), &crc_data)?;

    Ok(())
}
