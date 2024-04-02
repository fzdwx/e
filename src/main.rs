use std::io::{Read, stdin};

fn main() {
    let mut buf = [0; 1024];
    loop {
        let result = stdin().read(&mut buf);
        match result {
            Ok(0) => break,
            Ok(n) => {
                let s = String::from_utf8_lossy(&buf[..n]);
                print!("{}", s);
            }
            Err(e) => {
                eprintln!("error: {}", e);
                break;
            }
        }
    }
}
