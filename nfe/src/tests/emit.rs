//! Testes da tag <emit>

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn base() -> Result<(), String> {

    let f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;
    let emit = Nfe::try_from(f)?.emit;

    assert_eq!("06929383000163", emit.cnpj);
    assert_eq!("UMA RAZAO SOCIAL DE TESTE QUALQUER", emit.razao_social);
    assert_eq!(None, emit.nome_fantasia);
    assert_eq!("0018000762", emit.ie);
    assert_eq!(None, emit.iest);

    Ok(())
}
