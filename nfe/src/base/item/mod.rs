//! Detalhamento de produtos e serviços

use super::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

mod imposto;
mod produto;

pub use imposto::*;
pub use produto::*;

/// Item da nota
#[derive(Debug, PartialEq, Deserialize, Serialize)]
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o item")
    }
}
