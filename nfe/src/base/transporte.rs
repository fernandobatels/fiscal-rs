//! Informações sobre o transporte da nota

use super::Error;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

/// Transporte da nota
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename = "transp")]
pub struct Transporte {
    /// Modalidade do frete
    #[serde(rename = "$unflatten=modFrete")]
    pub modalidade: ModalidadeFrete,
}

impl FromStr for Transporte {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Transporte {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o transporte")
    }
}

/// Modalidade do frete
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
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
