use crate::oci_spec::{RuntimeSpecExt, RuntimeSpecUdev};
use clap::Parser;
use log::warn;
use oci_spec::runtime::Spec as RuntimeSpec;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[arg(short, long, value_name = "FILE")]
    pub spec_file: Option<PathBuf>,

    #[arg(short, long, help = "Verbosity level", action = clap::ArgAction::Count)]
    pub verbose: u8,
}

pub trait CLIExt {
    fn run<R: Read>(self, stdin: R, spec_from_stdin: bool) -> Result<(), Box<dyn Error>>;
}

impl CLIExt for CLI {
    fn run<R: Read>(self, stdin: R, spec_from_stdin: bool) -> Result<(), Box<dyn Error>> {
        let mut spec = match (spec_from_stdin, self.spec_file) {
            (true, path) => {
                if path.is_some() {
                    warn!("Ignoring spec file");
                }
                RuntimeSpec::from_reader(stdin)?
            }
            (false, Some(path)) => RuntimeSpec::load(path)?,
            _ => {
                let err = IoError::new(ErrorKind::InvalidInput, "No spec file provided");
                return Err(Box::new(err));
            }
        };

        let seat = if let Some(annotations) = spec.annotations() {
            annotations.get("io.dev-binder.udev.seat").cloned()
        } else {
            None
        };

        if seat.is_some() {
            let mut enumerator = udev::Enumerator::new()?;
            if let Some(seat_id) = &seat {
                enumerator.match_tag("seat")?;
                if seat_id != "seat0" {
                    enumerator.match_tag(seat_id)?;
                }
            }

            for device in enumerator.scan_devices()? {
                // In udev, a device is assigned to seat0 if it has the property
                // ID_SEAT=seat0 or if it has no ID_SEAT property at all.
                // if seat_id is seat0 we'd want to skip devices that don't follow this rule.
                // for more info: https://www.freedesktop.org/wiki/Software/systemd/multiseat/
                if device.devnode().is_some() {
                    if let Some(seat_id) = &seat
                        && seat_id == "seat0"
                        && let Some(device_seat_id) = device.property_value("ID_SEAT")
                        && seat_id.as_str() != device_seat_id
                    {
                        continue;
                    };

                    spec.add_udev_device(device)?
                }
            }
        }
        println!("{}", spec.to_string()?);
        Ok(())
    }
}

#[cfg(test)]
#[cfg(not(feature = "disable-udev-required-tests"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::cli::{CLI, CLIExt};
    use std::io::{Seek, Write, stdin};
    use tempfile::NamedTempFile;

    #[test]
    fn test_run_with_spec_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut spec_file = NamedTempFile::new()?;
        spec_file.write_all(b"{}")?;

        let cli = CLI {
            spec_file: Some(spec_file.path().to_path_buf()),
            verbose: 0,
        };

        cli.run(stdin(), false)
    }

    #[test]
    fn test_run_with_stdin() -> Result<(), Box<dyn std::error::Error>> {
        let mut stdin_mock = NamedTempFile::new()?;
        stdin_mock.write_all(b"{}")?;
        stdin_mock.flush()?;
        stdin_mock.rewind()?;

        let cli = CLI {
            spec_file: None,
            verbose: 0,
        };
        cli.run(stdin_mock, true)
    }

    #[test]
    fn test_run_with_stdin_and_spec_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut spec_file = NamedTempFile::new()?;
        spec_file.write_all(b"{}")?;
        spec_file.flush()?;
        spec_file.rewind()?;

        let cli = CLI {
            spec_file: Some(spec_file.path().to_path_buf()),
            verbose: 0,
        };

        cli.run(spec_file, true)
    }

    #[test]
    fn test_run_cli_with_no_spec_file() {
        let cli = CLI {
            spec_file: None,
            verbose: 0,
        };

        assert!(cli.run(stdin(), false).is_err());
    }

    #[test]
    fn test_run_cli_with_no_stdin_spec_file() -> Result<(), Box<dyn std::error::Error>> {
        let stdin_mock = NamedTempFile::new()?;
        let cli = CLI {
            spec_file: None,
            verbose: 0,
        };

        assert!(cli.run(stdin_mock, false).is_err());
        Ok(())
    }
}
