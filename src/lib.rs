use anyhow::{anyhow, Result};
use log::debug;
use std::ffi::OsStr;
use std::process::Command;

/// Get exit code, print the output to stdout as it goes
pub fn getstatus<S, I>(exec: S, args: I) -> Result<i32>
where
    S: AsRef<OsStr>,
    I: IntoIterator,
    I::Item: AsRef<OsStr>,
{
    let mut process = Command::new(exec);
    for a in args {
        process.arg(a);
    }

    debug!("Running command: {:?}", process);

    let exit_status = process.status()?;
    let code = exit_status
        .code()
        .ok_or(anyhow!("Unable to get exit code"))?;
    Ok(code)
}

/// Same as getstatus, but wrap in a shell invocation
pub fn getstatus_shell<S: AsRef<OsStr>>(command: S) -> Result<i32> {
    getstatus("/bin/sh", [OsStr::new("-c"), OsStr::new(&command)])
}

/// Capture status and output
pub fn getstatusoutput<S, I>(exec: S, args: I) -> Result<(i32, Vec<u8>)>
where
    S: AsRef<OsStr>,
    I: IntoIterator,
    I::Item: AsRef<OsStr>,
{
    let mut process = Command::new(exec);
    for a in args {
        process.arg(a);
    }

    debug!("Running command: {:?}", process);

    let output = process.output()?;
    let code = output
        .status
        .code()
        .ok_or(anyhow!("Unable to get exit code"))?;

    Ok((code, output.stdout))
}

/// Same as getstatusoutput, but wrap in a shell invocation
pub fn getstatusoutput_shell<S: AsRef<OsStr>>(command: S) -> Result<(i32, Vec<u8>)> {
    getstatusoutput("/bin/sh", [OsStr::new("-c"), OsStr::new(&command)])
}

/// Return Err if non-zero exit code
pub fn run_shell_or_err<S: AsRef<OsStr> + std::fmt::Display>(command: S) -> Result<()> {
    match getstatus_shell(&command) {
        Ok(0) => return Ok(()),
        _ => Err(anyhow!("Error with command: {}", &command)),
    }
}

/// Return Err if non-zero exit code
pub fn getoutput_shell_or_err<S: AsRef<OsStr> + std::fmt::Display>(command: S) -> Result<Vec<u8>> {
    match getstatusoutput_shell(&command) {
        Ok((0, output)) => return Ok(output),
        Ok((code, output)) => Err(anyhow!(
            "Error with command: Command: {}\nCode: {}\nOutput: {:?}",
            &command,
            code,
            output
        )),
        _ => Err(anyhow!("Error with command: {}", &command)),
    }
}
