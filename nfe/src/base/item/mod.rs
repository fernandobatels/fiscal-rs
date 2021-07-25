//! Detalhamento de produtos e serviços

use serde::Deserialize;
use std::str::FromStr;

mod imposto;
mod produto;

pub use imposto::*;
pub use produto::*;

/// Item da nota
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename = "det")]
pub struct Item {
    #[serde(rename = "nItem")]
    pub numero: u8,
    #[serde(rename = "prod")]
    pub produto: Produto,
    #[serde(rename = "imposto")]
    pub imposto: Imposto,
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s).map_err(|e| e.to_string())
    }
}
