//! Testes da seção de informações sobre o transporte

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn modalidade() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let transporte = Nfe::try_from(f)?.transporte;

    assert_eq!(ModalidadeFrete::SemTransporte, transporte.modalidade);

    Ok(())
}
