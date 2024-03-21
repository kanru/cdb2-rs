use std::{io::Result, path::PathBuf};

use cdb2::CDB;

pub fn main() -> Result<()> {
    let flags = xflags::parse_or_exit! {
        /// CDB file path
        required db_path: PathBuf
    };

    let db = CDB::open(flags.db_path)?;
    if let Some(value) = db.get(b"apple") {
        let _ = value?;
    }
    for entry in db.find(b"12345678") {
        let _ = entry?;
    }
    for entry in db.iter() {
        let (key, value) = entry?;
        let _ = key;
        let _ = value;
    }

    Ok(())
}
