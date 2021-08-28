//! Endereço do emitente/destinatário da NF-e

use super::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Representação de um endereço usado na NFe
#[derive(Debug, Deserialize, PartialEq, Clone, Serialize)]
pub struct Endereco {
    #[serde(rename = "$unflatten=xLgr")]
    pub logradouro: String,
    #[serde(rename = "$unflatten=nro")]
    pub numero: String,
    #[serde(rename = "$unflatten=xCpl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complemento: Option<String>,
    #[serde(rename = "$unflatten=xBairro")]
    pub bairro: String,
    #[serde(rename = "$unflatten=cMun")]
    pub codigo_municipio: u32,
    #[serde(rename = "$unflatten=xMun")]
    pub nome_municipio: String,
    #[serde(rename = "$unflatten=UF")]
    pub sigla_uf: String,
    #[serde(rename = "$unflatten=CEP")]
    pub cep: String,
    #[serde(rename = "$unflatten=cPais")]
    pub codigo_pais: u32,
    #[serde(rename = "$unflatten=xPais")]
    pub nome_pais: String,
    #[serde(rename = "$unflatten=fone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefone: Option<String>,
}

impl FromStr for Endereco {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Endereco {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o endereço")
    }
}
