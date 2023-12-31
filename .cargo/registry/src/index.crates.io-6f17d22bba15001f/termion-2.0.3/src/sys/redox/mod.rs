extern crate libredox;
extern crate redox_termios;

use std::io;

pub use self::redox_termios::Termios;

pub mod attr;
pub mod size;
pub mod tty;

// Support function for converting syscall error to io error
fn cvt(result: Result<usize, libredox::error::Error>) -> io::Result<usize> {
    result.map_err(|err| io::Error::from_raw_os_error(err.errno))
}
