//! Testes da tag <dest>

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let dest = Nfe::try_from(f).map_err(|e| e.to_string())?.dest;

    assert_eq!("58716523000119", dest.cnpj);
    assert_eq!(
        "NF-E EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL",
        dest.razao_social
    );
    assert_eq!(IndicadorContribuicaoIe::Contribuinte, dest.indicador_ie);
    assert_eq!(Some("112006603110".to_string()), dest.ie);
    assert_eq!("Av. Teste", dest.endereco.logradouro);
    assert_eq!("2040", dest.endereco.numero);
    assert_eq!(None, dest.endereco.complemento);
    assert_eq!("Centro", dest.endereco.bairro);
    assert_eq!(3550308, dest.endereco.codigo_municipio);
    assert_eq!("SAO PAULO", dest.endereco.nome_municipio);
    assert_eq!("SP", dest.endereco.sigla_uf);
    assert_eq!("04207040", dest.endereco.cep);
    assert_eq!(Some("5190909090".to_string()), dest.endereco.telefone);

    Ok(())
}

#[test]
fn manual() -> Result<(), Error> {
    let xml = "
        <dest>
            <CNPJ>58716523000119</CNPJ>
            <xNome>NF-E EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL</xNome>
            <enderDest>
                <xLgr>Av. Teste</xLgr>
                <nro>2040</nro>
                <xBairro>Centro</xBairro>
                <cMun>3550308</cMun>
                <xMun>SAO PAULO</xMun>
                <UF>SP</UF>
                <CEP>04207040</CEP>
                <cPais>1058</cPais>
                <xPais>BRASIL</xPais>
                <fone>5190909090</fone>
            </enderDest>
            <indIEDest>1</indIEDest>
            <IE>112006603110</IE>
        </dest>
    ";

    let dest = xml.parse::<Destinatario>()?;

    assert_eq!("58716523000119", dest.cnpj);
    assert_eq!(
        "NF-E EMITIDA EM AMBIENTE DE HOMOLOGACAO - SEM VALOR FISCAL",
        dest.razao_social
    );
    assert_eq!(IndicadorContribuicaoIe::Contribuinte, dest.indicador_ie);
    assert_eq!(Some("112006603110".to_string()), dest.ie);
    assert_eq!("Av. Teste", dest.endereco.logradouro);
    assert_eq!("2040", dest.endereco.numero);
    assert_eq!(None, dest.endereco.complemento);
    assert_eq!("Centro", dest.endereco.bairro);
    assert_eq!(3550308, dest.endereco.codigo_municipio);
    assert_eq!("SAO PAULO", dest.endereco.nome_municipio);
    assert_eq!("SP", dest.endereco.sigla_uf);
    assert_eq!("04207040", dest.endereco.cep);
    assert_eq!(Some("5190909090".to_string()), dest.endereco.telefone);

    Ok(())
}

#[test]
fn to_string() -> Result<(), Error> {
    let mut xml_original = "
        <dest>
            <CNPJ>58716523000119</CNPJ>
            <xNome>HOMOLOGACAO</xNome>
            <enderDest>
                <xLgr>Av.Teste</xLgr>
                <nro>2040</nro>
                <xBairro>Centro</xBairro>
                <cMun>3550308</cMun>
                <xMun>Guarulhos</xMun>
                <UF>SP</UF>
                <CEP>04207040</CEP>
                <cPais>1058</cPais>
                <xPais>BRASIL</xPais>
                <fone>5190909090</fone>
            </enderDest>
            <IE>112006603110</IE>
            <indIEDest>1</indIEDest>
        </dest>
    "
    .to_string();
    xml_original.retain(|c| c != '\n' && c != ' ');

    let destinatario = xml_original.parse::<Destinatario>()?;
    let xml_novo = destinatario.to_string();

    assert_eq!(xml_original, xml_novo);

    Ok(())
}
