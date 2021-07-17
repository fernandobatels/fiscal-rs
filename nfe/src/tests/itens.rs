//! Testes dos itens/produtos da nf

use std::convert::TryFrom;
use std::fs::File;

use crate::*;
use crate::base::Nfe as NfeBase;

#[test]
fn base() -> Result<(), String> {

    let f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;
    let itens = Nfe::try_from(f)?.itens;

    assert_eq!(1, itens.len());

    let item = &itens[0];

    assert_eq!(1, item.numero);

    Ok(())
}

#[test]
fn produto() -> Result<(), String> {

    let f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;
    let itens = Nfe::try_from(f)?.itens;

    assert_eq!(1, itens.len());

    let produto = &itens[0].produto;

    assert_eq!("11007", produto.codigo);
    assert_eq!(None, produto.gtin);
    assert_eq!("UM PRODUTO TESTE QUALQUER", produto.descricao);
    assert_eq!("64011000", produto.ncm);
    assert_eq!(Some("1234567".to_string()), produto.cest);

    Ok(())
}

#[test]
fn produtos() -> Result<(), String> {

    let f = File::open("xmls/nfce_layout4.xml")
        .map_err(|e| e.to_string())?;
    let itens = NfeBase::try_from(f)?.itens;

    assert_eq!(2, itens.len());

    let produto = &itens[0].produto;

    assert_eq!("10015300336", produto.codigo);
    assert_eq!(Some("7893049207584".to_string()), produto.gtin);
    assert_eq!("(153 - C2075) -CILINDRO MESTRE DUPLO UN", produto.descricao);
    assert_eq!("87083090", produto.ncm);
    assert_eq!(None, produto.cest);

    let produto = &itens[1].produto;

    assert_eq!("10029200332", produto.codigo);
    assert_eq!(None, produto.gtin);
    assert_eq!("(292 - BAH0031D) -ROLAMENTO RODA DIANTEIRO SEM ABS UN", produto.descricao);
    assert_eq!("84821090", produto.ncm);
    assert_eq!(None, produto.cest);

    Ok(())
}