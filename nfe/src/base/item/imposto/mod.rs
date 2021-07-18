//! Impostos dos itens

mod icms;
mod pis;

pub use icms::*;
use parsercher::dom::*;
pub use pis::*;

/// Detalhamentos impostos sobre o item
pub struct Imposto {
    /// Valor aproximado total de tributos federais, estaduais e municipais
    pub valor_aproximado: Option<f32>,
    /// Informações do ICMS da Operação própria e ST
    pub icms: Option<GrupoIcms>,
    /// Informações do PIS
    pub pis: Option<GrupoPis>,
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

        Ok(Imposto {
            valor_aproximado,
            icms,
            pis,
        })
    }
}
