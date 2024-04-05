use anyhow::Result;
use ropey::RopeSlice;
use std::io::Write;

pub fn write_slices<F: Write>(f: &mut F, rope_slice: RopeSlice, col_offset: usize) -> Result<()> {
    let chars = rope_slice.chars();
    let len = chars.len();
    if len == 0 {
        return Ok(());
    }
    if len <= col_offset {
        return Ok(());
    }

    let mut total = len - col_offset;
    let mut col_offset = col_offset;
    for x in chars {
        if x == '\n' {
            break;
        }
        if total == 0 {
            break;
        }

        if col_offset > 0 {
            col_offset -= 1;
            continue;
        }
        total -= 1;
        write!(f, "{}", x)?;
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
