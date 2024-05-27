use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Args(String, String),
    Conversion(String, String),
    Heapify(String, String),
    Encoding(String, String),
    Compress(String, String),
    DeCompress(String, String),
    Finalizing(String, String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Conversion(msg, err) => f.write_fmt(format_args!("[ERROR] - conversion: {}; mapped: {};", msg, err)),
            Error::Heapify(msg, err) => f.write_fmt(format_args!("[ERROR] - heapify: {}; mapped: {};", msg, err)),
            Error::Encoding(msg, err) => f.write_fmt(format_args!("[ERROR] - encoding: {}; mapped: {};", msg, err)),
            Error::Args(msg, err) => f.write_fmt(format_args!("[ERROR] - input args: {}; mapped: {};", msg, err)),
            Error::Compress(msg, err) => f.write_fmt(format_args!("[ERROR] - compress: {}; mapped: {};", msg, err)),
            Error::DeCompress(msg, err) => f.write_fmt(format_args!("[ERROR] - decompress: {}; mapped: {};", msg, err)),
            Error::Finalizing(msg, err) => f.write_fmt(format_args!("[ERROR] - finalizing: {}; mapped: {};", msg, err)),
        }
    }
}

impl std::error::Error for Error {}
