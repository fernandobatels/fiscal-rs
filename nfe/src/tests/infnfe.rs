//! Testes da tag <infNFe>

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn nfe() -> Result<(), String> {

    let f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;
    let nfe = Nfe::try_from(f)?;

    assert_eq!("43180906929383000163550010000000261000010301", nfe.chave_acesso);
    assert_eq!(VersaoLayout::V4_00, nfe.versao);

    Ok(())
}
