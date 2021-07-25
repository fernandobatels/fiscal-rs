//! Informações sobre o transporte da nota

use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::str::FromStr;

/// Transporte da nota
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "transp")]
pub struct Transporte {
    /// Modalidade do frete
    #[serde(rename = "modFrete")]
    pub modalidade: ModalidadeFrete,
}

impl FromStr for Transporte {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s).map_err(|e| e.to_string())
    }
}

/// Modalidade do frete
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum ModalidadeFrete {
    /// CIF - Contratação do frete por conta do remetente
    ContratacaoPorContaRemetente = 0,
    /// FOB - Contratação do frete por conta do destinarário
    ContratacaoPorContaDestinatario = 1,
    /// Contratação do frete por conta de terceiros
    ContratacaoPorContaTerceiros = 2,
    /// Transporte próprio por conta do remetente
    TransportePorContaRemetente = 3,
    /// Transporte próprio por conta do destinatário
    TransportePorContaDestinatario = 4,
    /// Sem ocorrência de transporte
    SemTransporte = 9,
}
