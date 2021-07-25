//! Impostos dos itens

use parsercher::dom::*;
use std::str::FromStr;
use serde::Deserialize;

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

impl Imposto {
    /// Parse dos impostos do item
    pub(crate) fn parse(xml: &Dom) -> Result<Imposto, String> {
        let mut t_imposto = Dom::new(DomType::Tag);
        t_imposto.set_tag(Tag::new("imposto"));

        let imposto =
            parsercher::search_dom(&xml, &t_imposto).ok_or("Tag <imposto> não encontrada")?;

        let valor_aproximado = {
            if let Some(vl) =
                parsercher::search_text_from_tag_children(&imposto, &Tag::new("vTotTrib"))
            {
                Some(vl[0].parse::<f32>().map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let icms = GrupoIcms::parse(&imposto)?;

        let pis = GrupoPis::parse(&imposto)?;

        let cofins = GrupoCofins::parse(&imposto)?;

        Ok(Imposto {
            valor_aproximado,
            icms,
            pis,
            cofins,
        })
    }
}

impl FromStr for Imposto {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s)
            .map_err(|e| e.to_string())
    }
}
