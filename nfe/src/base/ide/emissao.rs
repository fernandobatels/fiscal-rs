//! Dados da emissão da NF-e

use super::Error;
use chrono::prelude::*;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::str::FromStr;

/// Dados referentes a emissão da nota
#[derive(Debug, Deserialize, PartialEq)]
pub struct Emissao {
    #[serde(rename = "dhEmi")]
    pub horario: DateTime<Utc>,
    #[serde(rename = "tpEmis")]
    pub tipo: TipoEmissao,
    #[serde(rename = "finNFe")]
    pub finalidade: FinalidadeEmissao,
    #[serde(rename = "procEmi")]
    pub processo: TipoProcessoEmissao,
    #[serde(rename = "verProc")]
    pub versao_processo: String,
}

/// Tipo da emissão da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
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
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum FinalidadeEmissao {
    Normal = 1,
    Complementar = 2,
    Ajuste = 3,
    Devolucao = 4,
}

/// Tipo do processo de emissão
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
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

impl FromStr for Emissao {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s).map_err(|e| e.into())
    }
}
