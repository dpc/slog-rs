//! Syslog drain for slog-rs
//!
//! WARNING: This crate needs some improvements.
//!
//! ```norust
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_syslog;
//!
//! use slog::*;
//! use slog_syslog::Facility;
//!
//! fn main() {
//!     let root = Logger::new_root(o!("build-id" => "8dfljdf"));
//!     root.set_drain(
//!             slog_syslog::unix_3164(
//!                 Facility::LOG_USER,
//!                 )
//!             );
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate slog_stream;
extern crate syslog;
extern crate nix;

use slog_stream::Format;
use slog::{Drain, Level, Record, OwnedKeyValueList};
use slog::ser::Serializer;
use std::{io, fmt};
use std::sync::Mutex;
use std::cell::RefCell;

use slog::ser;

pub use syslog::Facility;

thread_local! {
    static TL_BUF: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(128))
}

fn level_to_severity(level: slog::Level) -> syslog::Severity {
    match level {
        Level::Critical => syslog::Severity::LOG_CRIT,
        Level::Error => syslog::Severity::LOG_ERR,
        Level::Warning => syslog::Severity::LOG_WARNING,
        Level::Info => syslog::Severity::LOG_NOTICE,
        Level::Debug => syslog::Severity::LOG_INFO,
        Level::Trace => syslog::Severity::LOG_DEBUG,
    }

}

/// Drain formatting records and writing them to a syslog ``Logger`
///
/// Uses mutex to serialize writes.
/// TODO: Add one that does not serialize?
pub struct Streamer3164 {
    io: Mutex<Box<syslog::Logger>>,
    format: Format3164,
}

impl Streamer3164 {
    /// Create new syslog ``Streamer` using given `format`
    pub fn new(logger: Box<syslog::Logger>) -> Self {
        Streamer3164 {
            io: Mutex::new(logger),
            format: Format3164::new(),
        }
    }
}

impl Drain for Streamer3164 {
    type Error = io::Error;

    fn log(&self, info: &Record, logger_values: &OwnedKeyValueList) -> io::Result<()> {

        TL_BUF.with(|buf| {
            let mut buf = buf.borrow_mut();
            let res = {
                || {
                    try!(self.format.format(&mut *buf, info, logger_values));
                    let sever = level_to_severity(info.level());
                    {
                        let io = try!(self.io
                            .lock()
                            .map_err(|_| io::Error::new(io::ErrorKind::Other, "locking error")));

                        let buf = String::from_utf8_lossy(&buf);
                        let buf = io.format_3164(sever, &buf).into_bytes();

                        let mut pos = 0;
                        while pos < buf.len() {
                            let n = try!(io.send_raw(&buf[pos..]));
                            if n == 0 {
                                break
                            }

                            pos += n;
                        }
                    }

                    Ok(())
                }
            }();
            buf.clear();
            res
        })
    }
}

/// Formatter to format defined in RFC 3164
pub struct Format3164;

impl Format3164 {
    /// Create new `Format3164`
    pub fn new() -> Self {
        Format3164
    }
}

impl slog_stream::Format for Format3164 {
    fn format(&self,
              io: &mut io::Write,
              rinfo: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        try!(write!(io, "{}", rinfo.msg()));

        let mut ser = KSV::new(io, ": ".into());
        {
            for (ref k, ref v) in logger_values.iter() {
                try!(ser.io().write_all(", ".as_bytes()));
                try!(v.serialize(rinfo, k, &mut ser));
            }

            for &(ref k, ref v) in rinfo.values().iter() {
                try!(ser.io().write_all(", ".as_bytes()));
                try!(v.serialize(rinfo, k, &mut ser));
            }
        }
        Ok(())
    }
}

/// Key-Separator-Value serializer
struct KSV<W: io::Write> {
    separator: String,
    io: W,
}

impl<W: io::Write> KSV<W> {
    fn new(io: W, separator: String) -> Self {
        KSV {
            io: io,
            separator: separator,
        }
    }

    fn io(&mut self) -> &mut W {
        &mut self.io
    }
}

impl<W: io::Write> Serializer for KSV<W> {
    fn emit_none(&mut self, key: &str) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, "None"));
        Ok(())
    }
    fn emit_unit(&mut self, key: &str) -> ser::Result {
        try!(write!(self.io, "{}", key));
        Ok(())
    }

    fn emit_bool(&mut self, key: &str, val: bool) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }

    fn emit_char(&mut self, key: &str, val: char) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }

    fn emit_usize(&mut self, key: &str, val: usize) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_str(&mut self, key: &str, val: &str) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> ser::Result {
        try!(write!(self.io, "{}{}{}", key, self.separator, val));
        Ok(())
    }
}

/// `Streamer` to Unix syslog using RFC 3164 format
pub fn unix_3164(facility: syslog::Facility) -> Streamer3164 {
    Streamer3164::new(syslog::unix(facility).unwrap())
}
