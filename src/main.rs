use core::iter::Iterator;
use std::{env, io};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use csv::ReaderBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut rdr = ReaderBuilder::new()
        .from_path(path)
        .unwrap();

    let h = rdr.headers().unwrap().clone();

    let rs = rdr.records().flatten();

    let stdout = io::stdout();
    let mut locked = stdout.lock();
    rs.for_each(|record| {
        let out: HashMap<&str, &str> = h.iter().zip(record.iter()).collect();
        let mut s = serde_json::to_string(&out).unwrap();
        s.push('\n');
        locked
            .write(s.as_bytes())
            .unwrap();
    });
}
