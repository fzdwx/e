use anyhow::Result;
use ropey::RopeSlice;
use std::io::Write;

pub fn write_slices<F: Write>(f: &mut F, rope_slice: RopeSlice) -> Result<()> {
    for chunk in rope_slice.chunks() {
        write!(f, "{}", &chunk[0..chunk.len() - 1])?
    }

    Ok(())
}
