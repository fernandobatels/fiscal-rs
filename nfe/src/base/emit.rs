//! Emitente da NF-e

use super::endereco::*;
use serde::Deserialize;
use std::str::FromStr;

/// Emitente da NF-e
#[derive(Debug, Deserialize, PartialEq)]
pub struct Emitente {
    #[serde(rename = "CNPJ")]
    pub cnpj: String,
    #[serde(rename = "xNome")]
    pub razao_social: String,
    #[serde(rename = "xFant")]
    pub nome_fantasia: Option<String>,
    #[serde(rename = "IE")]
    pub ie: String,
    #[serde(rename = "IEST")]
    pub iest: Option<u32>,
    #[serde(rename = "enderEmit")]
    pub endereco: Endereco,
}

impl FromStr for Emitente {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s).map_err(|e| e.to_string())
    }
}
