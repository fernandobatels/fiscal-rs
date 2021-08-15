//! Destinarário da NF-e

use super::endereco::*;
use super::Error;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

/// Destinatário base da NF-e
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "dest")]
pub struct Destinatario {
    #[serde(rename = "CNPJ")]
    pub cnpj: String,
    #[serde(rename = "xNome")]
    pub razao_social: Option<String>,
    #[serde(rename = "enderDest")]
    pub endereco: Option<Endereco>,
    #[serde(rename = "IE")]
    pub ie: Option<String>,
    #[serde(rename = "indIEDest")]
    pub indicador_ie: IndicadorContribuicaoIe,
}

/// Indicador da IE do destinatário
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum IndicadorContribuicaoIe {
    /// Contribuinte ICMS
    Contribuinte = 1,
    /// Contribuinte isento de Inscrição no cadastro de Contribuintes
    Isento = 2,
    /// Não Contribuinte, que pode ou não possuir Inscrição Estadual no Cadastro de Contribuintes do ICMS
    NaoContribuinte = 9,
}

impl FromStr for Destinatario {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Destinatario {
    fn to_string(&self) -> String {
        serde_xml_rs::to_string(self).expect("Falha ao serializar o destinatário")
    }
}