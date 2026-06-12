#![cfg(unix)]

use std::error::Error;

use rocksdb::{DB, Options};

use crate::cli::RockdbCmd;

pub(crate) fn run(cmd: RockdbCmd) -> Result<(), Box<dyn Error>> {
    match cmd {
        RockdbCmd::Demo => demo(),
    }
}

fn demo() -> Result<(), Box<dyn Error>> {
    let path = "_data/rockdb";
    std::fs::create_dir_all(path)?;

    let mut opts = Options::default();
    opts.create_if_missing(true);

    let db = DB::open(&opts, path)?;

    db.put(b"greeting", b"hello")?;
    db.put(b"count", b"42")?;
    db.put(b"binary", &[0x00, 0x01, 0x02, 0x03])?;
    println!("put 3 keys");

    if let Some(value) = db.get(b"greeting")? {
        println!("greeting = {}", String::from_utf8_lossy(&value));
    }
    if let Some(value) = db.get(b"count")? {
        println!("count = {}", String::from_utf8_lossy(&value));
    }
    if let Some(value) = db.get(b"binary")? {
        println!("binary = {:?}", value);
    }

    db.delete(b"count")?;
    println!("deleted count");

    let iter = db.iterator(rocksdb::IteratorMode::Start);
    for item in iter {
        let (k, v) = item?;
        println!("key={:?} value={:?}", k, v);
    }

    drop(db);
    Ok(())
}
