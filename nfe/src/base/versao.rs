//! Layouts suportados

use std::str::FromStr;
use serde::Deserialize;

/// Versão do layout da NF-e
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize)]
pub enum VersaoLayout {
    #[serde(rename = "4.00")]
    V4_00 = 4,
}

impl FromStr for VersaoLayout {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "4.00" => Ok(VersaoLayout::V4_00),
            _ => unreachable!()
        }
    }
}
