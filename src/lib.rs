//! Decompress `*.dbc` files usually found in Brazil's DATASUS [ftp server] into `*.dbf` files.
//!
//! The underlying decompression algorithm used in `*.dbc` files is the *implode* algorithm from the PKWARE Data Compression Library.
//! This library uses *Aaron Griffith*'s [rust implementation] of the *implode* algorithm. Also,
//! this library is heavily inspired by *Daniela Petruzalek*'s [pysus]. I want to thank both of them, without their work this library
//! would not be possible.
//!
//! [ftp server]: ftp://ftp.datasus.gov.br/dissemin/publicos
//! [rust implementation]: https://crates.io/crates/explode
//! [pysus]: https://github.com/danicat/pysus
//!
//! # Examples
//!
//! To decompress a `*.dbc` file into a `*.dbf` use [`decompress`](fn.decompress.html):
//! ```no_run
//! datasus_dbc::decompress("input.dbc", "output.dbf");
//! ```
//!
//! ---
//!
//! If you want more control over how the `*.dbc` file is read,
//! you can pass a [`File`][File] or other type which implements [`Read`][Read] to [`into_dbf_reader`](fn.into_dbf_reader.html)
//! to get a reader of the decompressed content.
//! ```no_run
//! use std::io::Read;
//!
//! let dbc_file = std::fs::File::open("input.dbc").unwrap();
//! let mut dbf_reader = datasus_dbc::into_dbf_reader(dbc_file).unwrap();
//! let mut buf: Vec<u8> = Default::default();
//! dbf_reader.read_to_end(&mut buf).unwrap();
//! println!("{:?}", &buf[0..20]);
//! ```
//!
//! [Read]: https://doc.rust-lang.org/std/io/trait.Read.html
//! [File]: https://doc.rust-lang.org/std/io/struct.File.html
//!

mod decompress;
mod error;

pub use decompress::{decompress, into_dbf_reader};
pub use error::{Error, Result};
