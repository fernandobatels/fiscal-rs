//! Testes da tags de endereço

use crate::*;

#[test]
fn emitente() -> Result<(), Error> {
    let xml = "<enderEmit>
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
    </enderEmit>";

    let endereco = xml.parse::<Endereco>()?;
    assert_eq!("Rua dos Testes", endereco.logradouro);
    assert_eq!("1020", endereco.numero);
    assert_eq!(Some("0".to_string()), endereco.complemento);
    assert_eq!("Centro", endereco.bairro);
    assert_eq!(4319901, endereco.codigo_municipio);
    assert_eq!("SAPIRANGA", endereco.nome_municipio);
    assert_eq!("RS", endereco.sigla_uf);
    assert_eq!("93800000", endereco.cep);
    assert_eq!(Some("5190909090".to_string()), endereco.telefone);

    Ok(())
}

#[test]
fn destinatario() -> Result<(), Error> {
    let xml = "<enderDest>
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
    </enderDest>";

    let endereco = xml.parse::<Endereco>()?;
    assert_eq!("Av. Teste", endereco.logradouro);
    assert_eq!("2040", endereco.numero);
    assert_eq!(None, endereco.complemento);
    assert_eq!("Centro", endereco.bairro);
    assert_eq!(3550308, endereco.codigo_municipio);
    assert_eq!("SAO PAULO", endereco.nome_municipio);
    assert_eq!("SP", endereco.sigla_uf);
    assert_eq!("04207040", endereco.cep);
    assert_eq!(Some("5190909090".to_string()), endereco.telefone);

    Ok(())
}

#[test]
fn to_string() -> Result<(), Error> {
    let mut xml_original = "<Endereco>
        <xLgr>Rua</xLgr>
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
    </Endereco>"
        .to_string();
    xml_original.retain(|c| c != '\n' && c != ' ');

    let endereco = xml_original.parse::<Endereco>()?;
    let xml_novo = endereco.to_string();

    assert_eq!(xml_original, xml_novo);

    Ok(())
}
