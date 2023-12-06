# datasus-dbc

[![crates.io](https://img.shields.io/crates/v/datasus-dbc.svg)](https://crates.io/crates/datasus-dbc)
[![docs.rs](https://docs.rs/datasus-dbc/badge.svg)](https://docs.rs/datasus-dbc)


Decompress `*.dbc` files usually found in Brazil's DATASUS [ftp server](ftp://ftp.datasus.gov.br/dissemin/publicos) into `*.dbf` files.

The underlying decompression algorithm used in `*.dbc` files is the *implode* algorithm from the PKWARE Data Compression Library. This library uses *Aaron Griffith*'s [rust implementation](https://crates.io/crates/explode) of the *implode* algorithm. Also, this library is heavily inspired by *Daniela Petruzalek*'s [pysus](https://github.com/danicat/pysus). I want to thank both of them, without their work this library would not be possible.

# Examples

To decompress a `*.dbc` file into a `*.dbf` use `decompress`:
```rust
datasus_dbc::decompress("input.dbc", "output.dbf");
```

---

If you want more control over how the `*.dbc` file is read, you can pass a [`File`](https://doc.rust-lang.org/std/io/struct.File.html) or other type which implements [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) to [`into_dbf_reader`](fn.into_dbf_reader.html) to get a reader of the decompressed content. 
```rust
use std::io::Read;

let dbc_file = std::fs::File::open("input.dbc").unwrap();
let mut dbf_reader = datasus_dbc::into_dbf_reader(dbc_file).unwrap();
let mut buf: Vec<u8> = Default::default();
dbf_reader.read_to_end(&mut buf).unwrap();
println!("{:?}", &buf[0..20]);
```

# Found a bug?
Feel free to create an issue [here](https://github.com/mymatsubara/datasus-dbc/issues/new) if you found a bug or if you want a new feature!
