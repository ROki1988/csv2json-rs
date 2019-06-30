use core::iter::Iterator;
use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut rdr = ReaderBuilder::new().from_path(path).unwrap();

    let h = rdr.headers().unwrap().clone();

    let rs = rdr.records().flatten();

    let stdout = io::stdout();
    let mut locked = stdout.lock();
    let ss = rs.map(|record: StringRecord| {
        let vs = record.iter().map(|v| {
            serde_json::from_str(v).unwrap_or_else(|_| serde_json::Value::String(v.to_owned()))
        });
        let out: HashMap<&str, serde_json::Value> = h.iter().zip(vs).collect();
        let mut s = serde_json::to_string(&out).unwrap();
        s.push('\n');
        s
    });

    ss.for_each(|s| {
        locked.write(s.as_bytes()).unwrap();
    })
}
