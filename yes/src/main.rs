use std::io::BufWriter;
use std::io::Write;

const BUFFER_SIZE : usize = 1024 * 16;

fn main() {
    let output = std::env::args().nth(1).unwrap_or("y".into());

    let mut buf = BufWriter::with_capacity(BUFFER_SIZE, std::io::stdout());

    loop {
        writeln!(buf, "{}", output).unwrap();
    }
}
