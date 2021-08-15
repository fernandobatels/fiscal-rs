//! Testes da tag <infNFe>

use std::convert::TryFrom;
use std::fs::File;

use crate::base::Nfe as NfeBase;
use crate::*;

#[test]
fn nfe() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let nfe = Nfe::try_from(f).map_err(|e| e.to_string())?;

    assert_eq!(
        "43180906929383000163550010000000261000010301",
        nfe.chave_acesso
    );
    assert_eq!(VersaoLayout::V4_00, nfe.versao);
    assert_eq!(None, nfe.informacao_complementar);

    Ok(())
}

#[test]
fn informacao_complementar_from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfce_layout4.xml").map_err(|e| e.to_string())?;
    let nfe = NfeBase::try_from(f).map_err(|e| e.to_string())?;

    assert_eq!(
        Some("11899318;422-JERK DIONNY;CLIENTE RECUSOU INFORMAR CPF/CNPJ NO CUPOM".to_string()),
        nfe.informacao_complementar
    );

    Ok(())
}
