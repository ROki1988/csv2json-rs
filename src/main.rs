use core::iter::Iterator;
use std::{env, io};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use csv::{ReaderBuilder, StringRecord};

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
    let ss= rs.map(|record: StringRecord| {
        let vs = record.iter().map(|v| serde_json::from_str(v)).flatten();
        let out: HashMap<&str, serde_json::Value> = h.iter().zip(vs).collect();
        let mut s = serde_json::to_string(&out).unwrap();
        s.push('\n');
        s
    });

    ss.for_each(|s| {
        locked
            .write(s.as_bytes())
            .unwrap();
    })
}
