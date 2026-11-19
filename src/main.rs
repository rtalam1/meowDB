use std::{env, fs::File, io::Read};

use anyhow::{Context, Result};
use meow_db::header::DbHeader;

fn main() -> Result<()> {
    let path = env::args().nth(1).context("usage: meow_db <path-to-db>")?;

    let mut file = File::open(&path).with_context(|| format!("opening {path}"))?;
    let mut buf = [0u8; 100];
    file.read_exact(&mut buf)?;

    let header = DbHeader::parse(&buf)?;
    println!("{header:#?}");
    Ok(())
}