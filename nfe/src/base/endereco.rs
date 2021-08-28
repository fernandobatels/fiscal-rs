//! Endereço do emitente/destinatário da NF-e

use super::Error;
use serde::Deserialize;
use std::str::FromStr;

/// Representação de um endereço usado na NFe
#[derive(Debug, Deserialize, PartialEq)]
pub struct Endereco {
    #[serde(rename = "xLgr")]
    pub logradouro: String,
    #[serde(rename = "nro")]
    pub numero: String,
    #[serde(rename = "xCpl")]
    pub complemento: Option<String>,
    #[serde(rename = "xBairro")]
    pub bairro: String,
    #[serde(rename = "cMun")]
    pub codigo_municipio: u32,
    #[serde(rename = "xMun")]
    pub nome_municipio: String,
    #[serde(rename = "UF")]
    pub sigla_uf: String,
    #[serde(rename = "CEP")]
    pub cep: u32,
    #[serde(rename = "fone")]
    pub telefone: Option<String>,
}

impl FromStr for Endereco {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}
