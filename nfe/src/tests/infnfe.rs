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

#[test]
fn base_to_string() -> Result<(), String> {
    let f = File::open("xmls/nfce_layout4.xml").map_err(|e| e.to_string())?;
    let nfe = NfeBase::try_from(f).map_err(|e| e.to_string())?;

    let xml_novo = nfe.to_string();

    assert!(xml_novo.contains(
        "<infNFe versao=\"4.00\" Id=\"NFe29181033657677000156650010001654399001654399\">"
    ));
    println!("{}", xml_novo);
    assert!(xml_novo.contains("<infAdic><infCpl>11899318;422-JERK DIONNY;CLIENTE RECUSOU INFORMAR CPF/CNPJ NO CUPOM</infCpl></infAdic>"));

    Ok(())
}

#[test]
fn to_string() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let nfe = Nfe::try_from(f).map_err(|e| e.to_string())?;

    let xml_novo = nfe.to_string();

    assert!(xml_novo.contains(
        "<infNFe versao=\"4.00\" Id=\"NFe43180906929383000163550010000000261000010301\">"
    ));
    assert!(!xml_novo.contains("<infAdic>"));

    Ok(())
}

#[test]
fn parse_to_string_parse_to_string() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let nfe1 = Nfe::try_from(f).map_err(|e| e.to_string())?;
    let xml1 = nfe1.to_string();

    let nfe2 = xml1.parse::<Nfe>().map_err(|e| e.to_string())?;
    let xml2 = nfe2.to_string();

    assert_eq!(xml1, xml2);

    Ok(())
}

#[test]
fn base_parse_to_string_parse_to_string() -> Result<(), String> {
    let f = File::open("xmls/nfce_layout4.xml").map_err(|e| e.to_string())?;
    let nfe1 = NfeBase::try_from(f).map_err(|e| e.to_string())?;
    let xml1 = nfe1.to_string();

    let nfe2 = xml1.parse::<NfeBase>().map_err(|e| e.to_string())?;
    let xml2 = nfe2.to_string();

    assert_eq!(xml1, xml2);

    Ok(())
}
