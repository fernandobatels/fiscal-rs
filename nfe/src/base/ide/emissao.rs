//! Dados da emissão da NF-e

use chrono::prelude::*;
use parsercher::dom::*;
use std::str::FromStr;

/// Dados referentes a emissão da nota
pub struct Emissao {
    pub horario: DateTime<Utc>,
    pub tipo: TipoEmissao,
    pub finalidade: FinalidadeEmissao,
    pub processo: TipoProcessoEmissao,
    pub versao_processo: String,
}

impl Emissao {
    /// Parse dos campos da tag <ide> relacionados a emissão
    pub(crate) fn parse(ide: &Dom) -> Result<Emissao, String> {
        let horario = parsercher::search_text_from_tag_children(&ide, &Tag::new("dhEmi"))
            .ok_or("Tag <dhEmi> não encontrada na <ide>")?[0]
            .parse::<DateTime<Utc>>()
            .map_err(|e| e.to_string())?;

        let tipo = parsercher::search_text_from_tag_children(&ide, &Tag::new("tpEmis"))
            .ok_or("Tag <tpEmis> não encontrada na <ide>")?[0]
            .parse::<TipoEmissao>()?;

        let finalidade = parsercher::search_text_from_tag_children(&ide, &Tag::new("finNFe"))
            .ok_or("Tag <finNfe> não encontrada na <ide>")?[0]
            .parse::<FinalidadeEmissao>()?;

        let processo = parsercher::search_text_from_tag_children(&ide, &Tag::new("procEmi"))
            .ok_or("Tag <procEmi> não encontrada na <ide>")?[0]
            .parse::<TipoProcessoEmissao>()?;

        let versao_processo = parsercher::search_text_from_tag_children(&ide, &Tag::new("verProc"))
            .ok_or("Tag <verProc> não encontrada na <ide>")?[0]
            .to_string();

        Ok(Emissao {
            horario,
            tipo,
            finalidade,
            processo,
            versao_processo,
        })
    }
}

/// Tipo da emissão da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoEmissao {
    /// Emissão normal (não em contingência)
    Normal = 1,
    /// Contingência FS-IA, com impressão do DANFE em Formulário de Segurança - Impressor Autônomo
    ContigenciaFsIa = 2,
    /// Contingência SCAN (Sistema de Contingência do Ambiente Nacional)
    ContingenciaScan = 3,
    /// Contingência EPEC (Evento Prévio da Emissão em Contingência)
    ContigenciaEpec = 4,
    /// Contingência FS-DA, com impressão do DANFE em Formulário de Segurança - Documento Auxiliar
    ContigenciaFsDa = 5,
    /// Contingência SVC-AN (SEFAZ Virtual de Contingência do AN)
    ContigenciaSvcAn = 6,
    /// Contingência SVC-RS (SEFAZ Virtual de Contingência do RS)
    ContigenciaSvcRs = 7,
    /// Contingência off-line da NFC-e
    ContigenciaOfflineNfce = 9,
}

/// Finalidade da emissão da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FinalidadeEmissao {
    Normal = 1,
    Complementar = 2,
    Ajuste = 3,
    Devolucao = 4,
}

/// Tipo do processo de emissão
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoProcessoEmissao {
    /// Emissão de NF-e com aplicativo do contribuinte
    ViaAplicativoDoContribuinte = 0,
    /// Emissão de NF-e avulsa pelo Fisco
    AvulsaPeloFisco = 1,
    /// Emissão de NF-e avulsa, pelo contribuinte com seu certificado digital, através do site do Fisco
    AvulsaPeloContribuinte = 2,
    /// Emissão NF-e pelo contribuinte com aplicativo fornecido pelo Fisco
    ViaAplicativoDoFisco = 3,
}

impl FromStr for TipoEmissao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "2" => TipoEmissao::ContigenciaFsIa,
            "3" => TipoEmissao::ContingenciaScan,
            "4" => TipoEmissao::ContigenciaEpec,
            "5" => TipoEmissao::ContigenciaFsDa,
            "6" => TipoEmissao::ContigenciaSvcAn,
            "7" => TipoEmissao::ContigenciaSvcRs,
            "9" => TipoEmissao::ContigenciaOfflineNfce,
            _ => TipoEmissao::Normal, // 1
        })
    }
}

impl FromStr for FinalidadeEmissao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "4" => FinalidadeEmissao::Devolucao,
            "3" => FinalidadeEmissao::Ajuste,
            "2" => FinalidadeEmissao::Complementar,
            _ => FinalidadeEmissao::Normal, // 1
        })
    }
}

impl FromStr for TipoProcessoEmissao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "3" => TipoProcessoEmissao::ViaAplicativoDoFisco,
            "2" => TipoProcessoEmissao::AvulsaPeloContribuinte,
            "1" => TipoProcessoEmissao::AvulsaPeloFisco,
            _ => TipoProcessoEmissao::ViaAplicativoDoContribuinte, // 0
        })
    }
}
