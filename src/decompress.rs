use super::error::{Error, Result};
use explode::ExplodeReader;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Chain;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;

type DbfReader<R> = Chain<Chain<Cursor<[u8; 10]>, Cursor<Vec<u8>>>, ExplodeReader<R>>;

/// Decompress a .dbc file into a .dbf file
pub fn decompress<P>(dbc_path: P, dbf_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let dbc_file = File::open(dbc_path)?;
    let mut dbf_reader = into_dbf_reader(dbc_file)?;
    let mut dbf_file = OpenOptions::new().write(true).create(true).open(dbf_path)?;
    io::copy(&mut dbf_reader, &mut dbf_file)?;

    Ok(())
}

/// Transform a .dbc reader into a .dbf reader. Make sure `dbc_reader` starts at the beginning of the file.
pub fn into_dbf_reader<R>(mut dbc_reader: R) -> Result<DbfReader<R>>
where
    R: io::Read,
{
    let mut pre_header: [u8; 10] = Default::default();
    let mut crc32: [u8; 4] = Default::default();
    dbc_reader
        .read_exact(&mut pre_header)
        .map_err(|_| Error::MissingHeader)?;

    let header_size: usize = usize::from(pre_header[8]) + (usize::from(pre_header[9]) << 8);

    let mut header: Vec<u8> = vec![0; header_size - 10];
    dbc_reader
        .read_exact(&mut header)
        .map_err(|_| Error::InvalidHeaderSize)?;
    dbc_reader
        .read_exact(&mut crc32)
        .map_err(|_| Error::InvalidHeaderSize)?;

    // Create readers for each part of the file
    let pre_header_reader = Cursor::new(pre_header);
    let header_reader = Cursor::new(header);
    let compressed_content_reader = ExplodeReader::new(dbc_reader);

    let dbf_reader = pre_header_reader
        .chain(header_reader)
        .chain(compressed_content_reader);

    Ok(dbf_reader)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_decompress() -> Result<()> {
        let input = r"test\data\sids.dbc";
        let output = r"test\data\sids.dbf";
        let expected = r"test\data\expected-sids.dbf";

        decompress(input, output)?;

        let output_file = fs::read(output)?;
        let expected_file = fs::read(expected)?;
        fs::remove_file(output)?;

        assert_eq!(
            output_file, expected_file,
            "Decompressed .dbf is not equal to expected result"
        );

        Ok(())
    }
}
