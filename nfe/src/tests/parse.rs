//! Testes da API de parse da NF-e

use std::convert::TryFrom;
use std::str::FromStr;
use std::fs::File;
use std::io::Read;

use crate::*;
use crate::base::Nfe as NfeBase;

#[test]
fn nfe_from_str() -> Result<(), String> {

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
fn nfe_from_read() -> Result<(), String> {

    let f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;

    Nfe::try_from(f)?;

    Ok(())
}

#[test]
fn nfce_from_str() -> Result<(), String> {

    let mut f = File::open("xmls/nfce_layout4.xml")
        .map_err(|e| e.to_string())?;

    let mut xml = String::new();
    f.read_to_string(&mut xml)
        .map_err(|e| e.to_string())?;

    NfeBase::from_str(&xml)?;

    xml.parse::<NfeBase>()?;

    Ok(())
}

#[test]
fn nfce_from_read() -> Result<(), String> {

    let f = File::open("xmls/nfce_layout4.xml")
        .map_err(|e| e.to_string())?;

    NfeBase::try_from(f)?;

    Ok(())
}
