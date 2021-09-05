//! Testes da tag <ide>

use chrono::prelude::*;
use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let ide = Nfe::try_from(f).map_err(|e| e.to_string())?.ide;

    assert_eq!(43, ide.codigo_uf);
    assert_eq!(4307609, ide.codigo_municipio);
    assert_eq!(1, ide.serie);
    assert_eq!(26, ide.numero);
    assert_eq!(ModeloDocumentoFiscal::Nfe, ide.modelo);
    assert_eq!(FormatoImpressaoDanfe::NormalRetrato, ide.formato_danfe);
    assert_eq!(TipoAmbiente::Homologacao, ide.ambiente);
    assert_eq!("00001030", ide.chave.codigo);
    assert_eq!(1, ide.chave.digito_verificador);

    Ok(())
}

#[test]
fn manual() -> Result<(), Error> {
    let ide = XML_MANUAL.parse::<Identificacao>()?;

    assert_eq!(43, ide.codigo_uf);
    assert_eq!(4307609, ide.codigo_municipio);
    assert_eq!(1, ide.serie);
    assert_eq!(26, ide.numero);
    assert_eq!(ModeloDocumentoFiscal::Nfe, ide.modelo);
    assert_eq!(FormatoImpressaoDanfe::NormalRetrato, ide.formato_danfe);
    assert_eq!(TipoAmbiente::Homologacao, ide.ambiente);
    assert_eq!("00001030", ide.chave.codigo);
    assert_eq!(1, ide.chave.digito_verificador);

    Ok(())
}

#[test]
fn emissao_from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let emissao = Nfe::try_from(f).map_err(|e| e.to_string())?.ide.emissao;

    assert_eq!(Utc.ymd(2018, 09, 25).and_hms(3, 0, 0), emissao.horario);
    assert_eq!(TipoEmissao::Normal, emissao.tipo);
    assert_eq!(FinalidadeEmissao::Normal, emissao.finalidade);
    assert_eq!(
        TipoProcessoEmissao::ViaAplicativoDoContribuinte,
        emissao.processo
    );
    assert_eq!("fernando", emissao.versao_processo);

    Ok(())
}

#[test]
fn operacao_from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let operacao = Nfe::try_from(f).map_err(|e| e.to_string())?.ide.operacao;

    assert_eq!("Venda de producao do estabelecimento", operacao.natureza);
    assert_eq!(
        Some(Utc.ymd(2018, 09, 25).and_hms(18, 14, 0)),
        operacao.horario
    );
    assert_eq!(TipoOperacao::Saida, operacao.tipo);
    assert_eq!(DestinoOperacao::Interestadual, operacao.destino);
    assert_eq!(TipoConsumidor::Normal, operacao.consumidor);
    assert_eq!(TipoPresencaComprador::Presencial, operacao.presenca);
    assert_eq!(None, operacao.intermediador);

    Ok(())
}

#[test]
fn to_string() -> Result<(), Error> {
    let mut xml_original = XML_MANUAL.to_string();
    xml_original.retain(|c| c != '\n' && c != ' ');

    let ide = xml_original.parse::<Identificacao>()?;
    let xml_novo = ide.to_string();

    assert_eq!(xml_original, xml_novo);

    Ok(())
}

const XML_MANUAL: &str = "
    <ide>
        <cUF>43</cUF>
        <nNF>26</nNF>
        <serie>1</serie>
        <mod>55</mod>
        <cMunFG>4307609</cMunFG>
        <tpImp>1</tpImp>
        <tpAmb>2</tpAmb>
        <cNF>00001030</cNF>
        <cDV>1</cDV>
        <dhEmi>2018-09-25T03:00:00+00:00</dhEmi>
        <tpEmis>1</tpEmis>
        <finNFe>1</finNFe>
        <procEmi>0</procEmi>
        <verProc>fernando</verProc>
        <dhSaiEnt>2018-09-25T18:14:00+00:00</dhSaiEnt>
        <tpNF>1</tpNF>
        <idDest>2</idDest>
        <natOp>Venda de producao do estabelecimento</natOp>
        <indFinal>0</indFinal>
        <indPres>1</indPres>
    </ide>
";
