//! Dados da emissão da NF-e

use chrono::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Dados referentes a emissão da nota
#[derive(Debug, PartialEq, Clone)]
pub struct Emissao {
    pub horario: DateTime<Utc>,
    pub tipo: TipoEmissao,
    pub finalidade: FinalidadeEmissao,
    pub processo: TipoProcessoEmissao,
    pub versao_processo: String,
}

/// Tipo da emissão da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
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
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FinalidadeEmissao {
    Normal = 1,
    Complementar = 2,
    Ajuste = 3,
    Devolucao = 4,
}

/// Tipo do processo de emissão
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
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
