//! Testes da tag <ide>

use chrono::prelude::*;
use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn nfe() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let ide = Nfe::try_from(f)?.ide;

    assert_eq!(43, ide.codigo_uf);
    assert_eq!(4307609, ide.codigo_municipio);
    assert_eq!(1, ide.serie);
    assert_eq!(26, ide.numero);
    assert_eq!(ModeloDocumentoFiscal::Nfe, ide.modelo);
    assert_eq!(FormatoImpressaoDanfe::NormalRetrato, ide.formato_danfe);
    assert_eq!(TipoAmbiente::Homologacao, ide.ambiente);

    Ok(())
}

#[test]
fn chave_acesso() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let chave = Nfe::try_from(f)?.ide.chave;

    assert_eq!(1030, chave.codigo);
    assert_eq!(1, chave.digito_verificador);

    Ok(())
}

#[test]
fn emissao() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let emissao = Nfe::try_from(f)?.ide.emissao;

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
fn operacao() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let operacao = Nfe::try_from(f)?.ide.operacao;

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
