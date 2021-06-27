//! Testes da API de parse da NF-e

use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::*;

#[test]
fn from_str() -> Result<(), String> {

    let mut f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;

    let mut xml = String::new();
    f.read_to_string(&mut xml)
        .map_err(|e| e.to_string())?;

    Nfe::from_str(&xml)?;

    xml.parse::<Nfe>()?;

    Ok(())
}

#[test]
fn from_reader() -> Result<(), String> {

    let f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;

    let mut bfr = BufReader::new(f);

    Nfe::parse(bfr)?;

    Ok(())
}
