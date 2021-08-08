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
    assert_eq!(1030, ide.chave.codigo);
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
    assert_eq!(1030, ide.chave.codigo);
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
fn emissao_manual() -> Result<(), Error> {
    let emissao = XML_MANUAL.parse::<Emissao>()?;

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
fn operacao_manual() -> Result<(), Error> {
    let operacao = XML_MANUAL.parse::<Operacao>()?;

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

const XML_MANUAL: &str = "
    <ide>
        <cUF>43</cUF>
        <cNF>00001030</cNF>
        <natOp>Venda de producao do estabelecimento</natOp>
        <mod>55</mod>
        <serie>1</serie>
        <nNF>26</nNF>
        <dhEmi>2018-09-25T00:00:00-03:00</dhEmi>
        <dhSaiEnt>2018-09-25T15:14:00-03:00</dhSaiEnt>
        <tpNF>1</tpNF>
        <idDest>2</idDest>
        <cMunFG>4307609</cMunFG>
        <tpImp>1</tpImp>
        <tpEmis>1</tpEmis>
        <cDV>1</cDV>
        <tpAmb>2</tpAmb>
        <finNFe>1</finNFe>
        <indFinal>0</indFinal>
        <indPres>1</indPres>
        <procEmi>0</procEmi>
        <verProc>fernando</verProc>
    </ide>
";
