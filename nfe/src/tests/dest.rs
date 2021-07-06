//! Testes da tag <dest>

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn nfe() -> Result<(), String> {

    let f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;
    let dest = Nfe::try_from(f)?.dest;


    Ok(())
}

#[test]
fn nfce() -> Result<(), String> {

    let f = File::open("xmls/nfce_layout4.xml")
        .map_err(|e| e.to_string())?;
    let dest = Nfe::try_from(f)?.dest;


    Ok(())
}
