use anyhow::Result;
use ropey::RopeSlice;
use std::io::Write;

pub fn write_slices<F: Write>(f: &mut F, rope_slice: RopeSlice, col_offset: usize) -> Result<()> {
    let chars = rope_slice.chars();
    let len = chars.len();
    if len == 0 {
        return Ok(());
    }

    let end = len - col_offset;
    let last = len - 1;
    for (i, c) in chars.enumerate().skip(col_offset) {
        if i > end {
            break;
        }
        if i != last {
            write!(f, "{}", c)?;
        }
    }

    Ok(())
}

mod tests {
    use ropey::Rope;
    use std::fs;

    #[test]
    fn test_slices() {
        let text = Rope::from_str(
            r#"Hello, World!
我
        "#,
        );
        let lines = text.line(1);
        lines.chars().for_each(|c| {
            print!("{}_", c);
        });
    }
}
