use std::io::{Read, stdin};

fn main() {
    let mut buf = [0; 1024];
    loop {
        let result = stdin().read(&mut buf);
        match result {
            Ok(0) => break,
            Ok(2) => {
                match (&buf[..1])[0] as char {
                    'q' => {
                        break;
                    }
                    _ => {
                        p(&buf[..2]);
                    }
                }
            }
            Ok(n) => {
                p(&buf[..n])
            }
            Err(e) => {
                eprintln!("error: {}", e);
                break;
            }
        }
    }
}

fn p(buf: &[u8]) {
    println!("{}", String::from_utf8_lossy(buf));
}