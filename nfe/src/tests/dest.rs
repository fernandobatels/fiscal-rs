//! Testes da tag <dest>

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn base() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let dest = Nfe::try_from(f)?.dest;

    assert_eq!("58716523000119", dest.cnpj);
    assert_eq!(
        "NF-E EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL",
        dest.razao_social
    );
    assert_eq!(IndicadorContribuicaoIe::Contribuinte, dest.indicador_ie);
    assert_eq!(Some("112006603110".to_string()), dest.ie);

    Ok(())
}

#[test]
fn endereco() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let endereco = Nfe::try_from(f)?.dest.endereco;

    assert_eq!("Av. Teste", endereco.logradouro);
    assert_eq!("2040", endereco.numero);
    assert_eq!(None, endereco.complemento);
    assert_eq!("Centro", endereco.bairro);
    assert_eq!(3550308, endereco.codigo_municipio);
    assert_eq!("SAO PAULO", endereco.nome_municipio);
    assert_eq!("SP", endereco.sigla_uf);
    assert_eq!(04207040, endereco.cep);
    assert_eq!(Some("5190909090".to_string()), endereco.telefone);

    Ok(())
}
