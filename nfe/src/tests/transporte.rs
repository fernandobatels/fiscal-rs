//! Testes da seção de informações sobre o transporte

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let transporte = Nfe::try_from(f).map_err(|e| e.to_string())?.transporte;

    assert_eq!(ModalidadeFrete::SemTransporte, transporte.modalidade);

    Ok(())
}

#[test]
fn manual() -> Result<(), Error> {
    let xml = "<transp><modFrete>9</modFrete></transp>";

    let transporte = xml.parse::<Transporte>()?;

    assert_eq!(ModalidadeFrete::SemTransporte, transporte.modalidade);

    Ok(())
}

#[test]
fn to_string() -> Result<(), Error> {

    let mut xml_original = "<transp><modFrete>9</modFrete></transp>".to_string();
    xml_original.retain(|c| c != '\n' && c != ' ');

    let transporte = xml_original.parse::<Transporte>()?;
    let xml_novo = transporte.to_string();

    assert_eq!(xml_original, xml_novo);

    Ok(())
}
