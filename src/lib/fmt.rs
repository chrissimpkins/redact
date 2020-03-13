use std::path::PathBuf;
use std::process::{Command, Stdio};

use failure::{bail, format_err, Error, Fail};

use crate::lib::toolchains::Toolchain;

pub(crate) fn rustformat(toolchain: Toolchain, filepath: &PathBuf) -> Result<(), Error> {
    verify_rustfmt(toolchain)?;

    let toolchain_str = &format!("{}", toolchain);
    match Command::new("rustup")
        .args(&[
            "run",
            toolchain_str,
            "--",
            "rustfmt",
            "fmt",
            &filepath.to_string_lossy(),
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
    {
        Ok(exitstatus) => {
            if exitstatus.success() {
                return Ok(());
            }
            bail!(
                "rustfmt +{} failed with exit status code {}",
                toolchain_str,
                exitstatus
            )
        }
        Err(error) => Err(error.into()),
    }
}

pub(crate) fn check_rustformat(toolchain: Toolchain, filepath: &PathBuf) -> Result<(), Error> {
    verify_rustfmt(toolchain)?;

    let toolchain_str = &format!("{}", toolchain);
    match Command::new("rustup")
        .args(&[
            "run",
            toolchain_str,
            "--",
            "rustfmt",
            "fmt",
            "--check",
            &filepath.to_string_lossy(),
        ])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
    {
        Ok(exitstatus) => {
            if exitstatus.success() {
                return Ok(());
            }
            bail!(
                "rustfmt +{} failed with exit status code {}",
                toolchain_str,
                exitstatus
            )
        }
        Err(error) => Err(error.into()),
    }
}

fn verify_rustfmt(toolchain: Toolchain) -> Result<(), Error> {
    let toolchain_str = &format!("{}", toolchain);

    // check rustfmt --version to confirm installation in toolchain
    match Command::new("rustup")
        .args(&["run", toolchain_str, "--", "rustfmt", "--version"])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
    {
        Ok(status) if status.success() => return Ok(()),
        _ => (),
    };

    // if rustfmt not installed in toolchain, install it
    match Command::new("rustup")
        .args(&["component", "add", "rustfmt", "--toolchain", toolchain_str])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
    {
        Ok(status) => {
            if status.success() {
                return Ok(());
            } else {
                bail!(
                    "unable to install rustfmt in toolchain {}: failed with exit status code {}",
                    toolchain_str,
                    status
                )
            }
        }
        Err(error) => return Err(error.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(true, true);
    }
}
