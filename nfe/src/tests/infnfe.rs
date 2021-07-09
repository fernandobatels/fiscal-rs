//! Testes da tag <infNFe>

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

use crate::*;
use crate::base;

#[test]
fn nfe() -> Result<(), String> {

    let f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;
    let nfe = Nfe::try_from(f)?;

    assert_eq!("43180906929383000163550010000000261000010301", nfe.chave_acesso);
    assert_eq!(VersaoLayout::V4_00, nfe.versao);

    Ok(())
}

#[test]
fn nfce() -> Result<(), String> {

    let mut f = File::open("xmls/nfce_layout4.xml")
        .map_err(|e| e.to_string())?;

    let mut xml = String::new();
    f.read_to_string(&mut xml)
        .map_err(|e| e.to_string())?;

    // O NFC-e ainda não é suportado pela crate,
    // mas o seu parse usando a NfeBase deve funcionar
    let nfe = base::parse(&xml)?;

    assert_eq!("29181033657677000156650010001654399001654399", nfe.chave_acesso);
    assert_eq!(VersaoLayout::V4_00, nfe.versao);

    Ok(())
}
