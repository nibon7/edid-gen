#[macro_use]
extern crate bitflags;

mod cvtmode;
pub use cvtmode::{CvtMode, CvtModeBuilder};

use anyhow::{anyhow, Context};
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
    process::Command,
};
use tempfile::Builder;

/// EDID version enum (version 1.x)
pub enum Version {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
    V1_4,
}

impl Version {
    /// Major version
    pub fn major(&self) -> u8 {
        1
    }

    /// Minor Version
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

impl std::str::FromStr for Version {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.0" => Ok(Version::V1_0),
            "1.1" => Ok(Version::V1_1),
            "1.2" => Ok(Version::V1_2),
            "1.3" => Ok(Version::V1_3),
            "1.4" => Ok(Version::V1_4),
            _ => Err(anyhow!(
                "EDID version inavlid. Valid versions range from 1.0 to 1.4"
            )),
        }
    }
}

fn calculate_crc(data: &[u8]) -> u8 {
    let mut sum: u16 = 0;

    for i in data.iter() {
        sum += *i as u16;
    }

    (0x100 - sum % 0x100) as u8
}

/// Generate binary EDID file
pub fn generate_edid(
    mode: &CvtMode,
    version: Version,
    timing_name: &str,
    output: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let dir = Builder::new()
        .prefix("edid")
        .tempdir()
        .context("Failed to create temporary directory")?;

    let edid_temp_path = dir.path().join("edid.S.template");
    let edid_temp_asm = include_bytes!("edid.S.template");
    std::fs::write(&edid_temp_path, edid_temp_asm)
        .with_context(|| format!("Failed to save {}", edid_temp_path.display()))?;

    let edid_path = dir.path().join("edid.S");
    let edid_asm = mode.generate_edid_asm(version, timing_name);
    std::fs::write(&edid_path, edid_asm.as_bytes())
        .with_context(|| format!("Failed to save {}", edid_path.display()))?;

    let status = Command::new("cc")
        .arg("-c")
        .arg(&edid_path)
        .arg("-o")
        .arg(output.as_ref())
        .status()
        .with_context(|| format!("Failed to compile {}", edid_path.display()))?;

    if !status.success() {
        return Err(anyhow!(format!(
            "Failed to compile {}",
            edid_path.display()
        )));
    }

    let status = Command::new("objcopy")
        .arg("-O")
        .arg("binary")
        .arg("-j")
        .arg(".data")
        .arg(output.as_ref())
        .status()
        .with_context(|| format!("Failed to objcopy {}", output.as_ref().display()))?;

    if !status.success() {
        return Err(anyhow!(format!(
            "Failed to objcopy {}",
            output.as_ref().display()
        )));
    }

    let mut output_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(output.as_ref())
        .with_context(|| {
            format!(
                "Failed to open {} with read/write mode",
                output.as_ref().display()
            )
        })?;

    let mut data: Vec<u8> = Vec::new();
    output_file
        .read_to_end(&mut data)
        .with_context(|| format!("Failed to read {}", output.as_ref().display()))?;

    let crc = calculate_crc(&data);

    output_file
        .seek(SeekFrom::End(-1))
        .with_context(|| format!("Failed to seek to the end of {}", output.as_ref().display()))?;

    output_file
        .write(&[crc])
        .with_context(|| format!("Failed to save {}", output.as_ref().display()))?;

    Ok(())
}
