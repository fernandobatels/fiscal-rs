//! Dados da operação da NF-e

use chrono::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Dados referentes a operação da nota
#[derive(Debug, PartialEq, Clone)]
pub struct Operacao {
    pub horario: Option<DateTime<Utc>>,
    pub tipo: TipoOperacao,
    pub destino: DestinoOperacao,
    pub natureza: String,
    pub consumidor: TipoConsumidor,
    pub presenca: TipoPresencaComprador,
    pub intermediador: Option<TipoIntermediador>,
}

/// Tipo de operação da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum TipoOperacao {
    Entrada = 0,
    Saida = 1,
}

/// Destino da operação da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum DestinoOperacao {
    Interna = 1,
    Interestadual = 2,
    ComExterior = 3,
}

/// Tipo do consumidor da NF-e
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum TipoConsumidor {
    Normal = 0,
    Final = 1,
}

/// Tipo da presença do comprador
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum TipoPresencaComprador {
    /// Não se aplica. Ex.: Nota complementar ou de ajuste
    NaoSeAplica = 0,
    /// Operação presencial
    Presencial = 1,
    /// Operação não presencial, via internet
    ViaInternel = 2,
    /// Operação não presencial, via teleatendimento
    ViaTeleatendimento = 3,
    /// NFC-e em operação com entrega a domicílio
    NfceEmDomicilio = 4,
    /// Operação presencial, fora do estabelecimento
    PresencialForaDoEstabelecimento = 5,
    /// Operação não presencial
    Outros = 9,
}

/// Tipo do intermediador
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum TipoIntermediador {
    /// Operação sem intermediador (em site ou plataforma própria)
    SemIntermediador = 0,
    /// Operação em site ou plataforma de terceiros (intermediadores/marketplace)
    EmSiteDeTerceiros = 1,
}
