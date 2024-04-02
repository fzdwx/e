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
                        print(&buf[..2]);
                    }
                }
            }
            Ok(n) => {
                print(&buf[..n])
            }
            Err(e) => {
                eprintln!("error: {}", e);
                break;
            }
        }
    }
}

fn print(buf: &[u8]) {
    for b in buf {
        print!("{}", *b as char);
    }
}