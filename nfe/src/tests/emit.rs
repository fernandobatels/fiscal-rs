//! Testes da tag <emit>

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let emit = Nfe::try_from(f).map_err(|e| e.to_string())?.emit;

    assert_eq!("06929383000163", emit.cnpj);
    assert_eq!("UMA RAZAO SOCIAL DE TESTE QUALQUER", emit.razao_social);
    assert_eq!(None, emit.nome_fantasia);
    assert_eq!("0018000762", emit.ie);
    assert_eq!(None, emit.iest);
    assert_eq!("Rua dos Testes", emit.endereco.logradouro);
    assert_eq!("1020", emit.endereco.numero);
    assert_eq!(Some("0".to_string()), emit.endereco.complemento);
    assert_eq!("Centro", emit.endereco.bairro);
    assert_eq!(4319901, emit.endereco.codigo_municipio);
    assert_eq!("SAPIRANGA", emit.endereco.nome_municipio);
    assert_eq!("RS", emit.endereco.sigla_uf);
    assert_eq!("93800000", emit.endereco.cep);
    assert_eq!(Some("5190909090".to_string()), emit.endereco.telefone);

    Ok(())
}

#[test]
fn manual() -> Result<(), Error> {
    let xml = "
        <emit>
            <CNPJ>06929383000163</CNPJ>
            <xNome>UMA RAZAO SOCIAL DE TESTE QUALQUER</xNome>
            <enderEmit>
                <xLgr>Rua dos Testes</xLgr>
                <nro>1020</nro>
                <xCpl>0</xCpl>
                <xBairro>Centro</xBairro>
                <cMun>4319901</cMun>
                <xMun>SAPIRANGA</xMun>
                <UF>RS</UF>
                <CEP>93800000</CEP>
                <cPais>1058</cPais>
                <xPais>BRASIL</xPais>
                <fone>5190909090</fone>
            </enderEmit>
            <IE>0018000762</IE>
            <CRT>1</CRT>
        </emit>
    ";

    let emit = xml.parse::<Emitente>()?;

    assert_eq!("06929383000163", emit.cnpj);
    assert_eq!("UMA RAZAO SOCIAL DE TESTE QUALQUER", emit.razao_social);
    assert_eq!(None, emit.nome_fantasia);
    assert_eq!("0018000762", emit.ie);
    assert_eq!(None, emit.iest);
    assert_eq!("Rua dos Testes", emit.endereco.logradouro);
    assert_eq!("1020", emit.endereco.numero);
    assert_eq!(Some("0".to_string()), emit.endereco.complemento);
    assert_eq!("Centro", emit.endereco.bairro);
    assert_eq!(4319901, emit.endereco.codigo_municipio);
    assert_eq!("SAPIRANGA", emit.endereco.nome_municipio);
    assert_eq!("RS", emit.endereco.sigla_uf);
    assert_eq!("93800000", emit.endereco.cep);
    assert_eq!(Some("5190909090".to_string()), emit.endereco.telefone);

    Ok(())
}

#[test]
fn to_string() -> Result<(), Error> {
    let mut xml_original = "
        <emit>
            <CNPJ>06929383000163</CNPJ>
            <xNome>QUALQUER</xNome>
            <IE>0018000762</IE>
            <enderEmit>
                <xLgr>Testes</xLgr>
                <nro>1020</nro>
                <xCpl>0</xCpl>
                <xBairro>Centro</xBairro>
                <cMun>4319901</cMun>
                <xMun>SAPIRANGA</xMun>
                <UF>RS</UF>
                <CEP>93800000</CEP>
                <cPais>1058</cPais>
                <xPais>BRASIL</xPais>
                <fone>5190909090</fone>
            </enderEmit>
        </emit>
    "
    .to_string();
    xml_original.retain(|c| c != '\n' && c != ' ');

    let emitente = xml_original.parse::<Emitente>()?;
    let xml_novo = emitente.to_string();

    assert_eq!(xml_original, xml_novo);

    Ok(())
}
