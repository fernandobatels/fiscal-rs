//! Layouts suportados

use std::str::FromStr;

/// Versão do layout da NF-e
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum VersaoLayout {
    V4_00 = 4,
}

impl FromStr for VersaoLayout {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "4.00" => Ok(VersaoLayout::V4_00),
            _ => Err(format!("Layout {} ainda não suportado", s)),
        }
    }
}
