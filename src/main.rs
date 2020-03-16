extern crate libc;

use libc::{c_char, TIOCSTI};
use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

use nix::ioctl_write_ptr_bad;
use structopt::StructOpt;

ioctl_write_ptr_bad!(tiocsti_write, TIOCSTI, c_char);

#[derive(Debug, StructOpt)]
#[structopt(name = "termecho", about = "Send commands and write data to tty/pts")]
struct Opt {
    #[structopt(short = "d", long = "device", required = true)]
    devices: Vec<PathBuf>,
    #[structopt(short = "c", required = true)]
    cmd: Vec<String>,
    #[structopt(
        short = "n",
        help = "Write without a trailing newline e.g. will not execute command"
    )]
    no_newline: bool,
}

// TODO: Handle errors properly e.g. ENOTTY, EPERM etc
fn termecho<P: AsRef<Path>>(devpath: &P, s: &CString) {
    let dev = OpenOptions::new()
        .read(false)
        .write(true)
        .open(devpath)
        .expect("Error opening device");

    let fd = dev.as_raw_fd();
    let mut p = s.as_ptr();

    while unsafe { *p } != b'\0' as c_char {
        unsafe {
            tiocsti_write(fd, p).unwrap();
            p = p.add(1);
        }
    }
    Ok(())
}

// TODO: Expose functionality as a library
fn main() {
    let opt = Opt::from_args();
    let s = CString::new(opt.cmd.join(" ") + if opt.no_newline { "" } else { "\n" })
        .expect("cmd cannot be parsed to a valid CString");
    for devpath in opt.devices {
        termecho(&devpath, &s);
    }
}
