extern crate libc;

use libc::{c_char, TIOCSTI};
use nix;
use nix::errno::Errno;
use nix::ioctl_write_ptr_bad;
use std::ffi::{CString, NulError};
use std::fmt;
use std::fs::OpenOptions;
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

ioctl_write_ptr_bad!(tiocsti_write, TIOCSTI, c_char);

#[derive(Debug, StructOpt)]
#[structopt(name = "termecho", about = "Send commands and write data to tty/pts")]
struct Opt {
    /// tty/pts devices to send to
    #[structopt(required = true)]
    devices: Vec<PathBuf>,
    /// Command to send/data to write. A trailing newline is inserted by
    /// default e.g. <cmd> will be execute. See -n flag to disable this.
    #[structopt(required = true, last = true)]
    cmd: Vec<String>,
    /// Write <cmd> without a trailing newline e.g. will not execute command.
    #[structopt(short = "n")]
    no_newline: bool,
}

#[derive(Debug)]
enum Error {
    InvalidPermissions,
    NoTTY,
    IO(io::Error),
    Nix(nix::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Re-use debug for simplicity
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<nix::Error> for Error {
    fn from(e: nix::Error) -> Error {
        match e {
            nix::Error::Sys(Errno::EPERM) => Error::InvalidPermissions,
            nix::Error::Sys(Errno::ENOTTY) => Error::NoTTY,
            _ => Error::Nix(e),
        }
    }
}

// Default implementations are sufficient here
impl std::error::Error for Error {}

fn termecho<P: AsRef<Path>>(devpath: &P, s: &CString) -> Result<(), Error> {
    let dev = OpenOptions::new().read(false).write(true).open(devpath)?;

    let fd = dev.as_raw_fd();
    let mut p = s.as_ptr();

    while unsafe { *p } != b'\0' as c_char {
        unsafe {
            tiocsti_write(fd, p)?;
            p = p.add(1);
        }
    }
    Ok(())
}

fn parse_cmd(cmd: Vec<String>, no_newline: bool) -> Result<CString, NulError> {
    CString::new(cmd.join(" ") + if no_newline { "" } else { "\n" })
}

// TODO: Expose functionality as a library
fn main() {
    let opt = Opt::from_args();
    let s = parse_cmd(opt.cmd, opt.no_newline).expect("<cmd> cannot be parsed to a valid CString");
    for devpath in opt.devices {
        let devpath_str = format!("{}", devpath.display());
        match termecho(&devpath, &s) {
            Err(Error::IO(_)) => eprintln!("Error opening {}", devpath_str),
            Err(Error::InvalidPermissions) => {
                eprintln!("Cannot write to {} - are you root?", devpath_str)
            }
            Err(Error::NoTTY) => eprintln!("{} is not a TTY/PTY", devpath_str),
            Err(Error::Nix(e)) => eprintln!("Unable to write to {}: {:?}", devpath_str, e),
            Ok(_) => println!("{} OK", devpath_str),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn parse_cmd_newline() {
        let cmd = parse_cmd(vec_of_strings!["ls", "-al"], false).unwrap();
        assert_eq!(cmd, CString::new("ls -al\n").unwrap());
    }

    #[test]
    fn parse_cmd_no_newline() {
        let cmd = parse_cmd(vec_of_strings!["ls", "-al"], true).unwrap();
        assert_eq!(cmd, CString::new("ls -al").unwrap());
    }

    #[test]
    fn parse_cmd_invalid_cstring() {
        assert!(parse_cmd(vec_of_strings!["ls\0", "-al"], true).is_err());
    }
}
