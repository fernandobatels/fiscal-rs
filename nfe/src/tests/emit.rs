//! Testes da tag <emit>

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn base() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let emit = Nfe::try_from(f)?.emit;

    assert_eq!("06929383000163", emit.cnpj);
    assert_eq!("UMA RAZAO SOCIAL DE TESTE QUALQUER", emit.razao_social);
    assert_eq!(None, emit.nome_fantasia);
    assert_eq!("0018000762", emit.ie);
    assert_eq!(None, emit.iest);

    Ok(())
}

#[test]
fn endereco() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let endereco = Nfe::try_from(f)?.emit.endereco;

    assert_eq!("Rua dos Testes", endereco.logradouro);
    assert_eq!("1020", endereco.numero);
    assert_eq!(Some("0".to_string()), endereco.complemento);
    assert_eq!("Centro", endereco.bairro);
    assert_eq!(4319901, endereco.codigo_municipio);
    assert_eq!("SAPIRANGA", endereco.nome_municipio);
    assert_eq!("RS", endereco.sigla_uf);
    assert_eq!(93800000, endereco.cep);
    assert_eq!(Some("5190909090".to_string()), endereco.telefone);

    Ok(())
}
