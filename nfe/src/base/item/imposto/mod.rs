//! Impostos dos itens

use super::Error;
use serde::Deserialize;
use std::str::FromStr;

mod cofins;
mod icms;
mod pis;

pub use cofins::*;
pub use icms::*;
pub use pis::*;

/// Detalhamentos impostos sobre o item
#[derive(Debug, Deserialize, PartialEq)]
pub struct Imposto {
    /// Valor aproximado total de tributos federais, estaduais e municipais
    #[serde(rename = "vTotTrib")]
    pub valor_aproximado: Option<f32>,
    /// Informações do ICMS da Operação própria e ST
    #[serde(rename = "ICMS")]
    pub icms: Option<GrupoIcms>,
    /// Informações do PIS
    #[serde(rename = "PIS")]
    pub pis: Option<GrupoPis>,
    /// Informações do COFINS
    #[serde(rename = "COFINS")]
    pub cofins: Option<GrupoCofins>,
}

impl FromStr for Imposto {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}