//! Emitente da NF-e

use super::endereco::*;
use super::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Emitente da NF-e
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename = "emit")]
pub struct Emitente {
    #[serde(rename = "$unflatten=CNPJ")]
    pub cnpj: String,
    #[serde(rename = "$unflatten=xNome")]
    pub razao_social: String,
    #[serde(rename = "$unflatten=xFant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nome_fantasia: Option<String>,
    #[serde(rename = "$unflatten=IE")]
    pub ie: String,
    #[serde(rename = "$unflatten=IEST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iest: Option<u32>,
    #[serde(rename = "enderEmit")]
    pub endereco: Endereco,
}

impl FromStr for Emitente {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Emitente {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o emitente")
    }
}
