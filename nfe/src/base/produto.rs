//! Produtos

use parsercher::dom::*;

/// Produto do item da nota
pub struct Produto {
    pub codigo: String,
    /// GTIN (Global Trade Item Number) do produto, antigo código EAN ou código de barras
    pub gtin: Option<String>,
    pub descricao: String,
    /// NCM - Nomenclatura Comum do Mercosul
    pub ncm: String,
    /// CEST - Código Especificador da Substituição    Tributária
    pub cest: Option<String>,
}

impl Produto {
    /// Parse do produto do item
    pub(crate) fn parse(xml: &Dom) -> Result<Produto, String> {

        let mut t_prod = Dom::new(DomType::Tag);
        t_prod.set_tag(Tag::new("prod"));

        let prod = parsercher::search_dom(&xml, &t_prod)
            .ok_or("Tag <prod> não encontrada")?;

        let codigo = parsercher::search_text_from_tag_children(&prod, &Tag::new("cProd"))
            .ok_or("Tag <cProd> não encontrada na <prod>")?[0]
            .to_string();

        let gtin = {
            if let Some(ean) = parsercher::search_text_from_tag_children(&prod, &Tag::new("cEAN")) {
                let gtin = ean[0].to_string();

                match gtin.to_lowercase().trim() {
                    "sem gtin" => None,
                    "sem ean" => None,
                    _ => Some(gtin)
                }
            } else {
                None
            }
        };

        let descricao = parsercher::search_text_from_tag_children(&prod, &Tag::new("xProd"))
            .ok_or("Tag <xProd> não encontrada na <prod>")?[0]
            .to_string();

        let ncm = parsercher::search_text_from_tag_children(&prod, &Tag::new("NCM"))
            .ok_or("Tag <NCM> não encontrada na <prod>")?[0]
            .to_string();

        let cest = {
            if let Some(ce) = parsercher::search_text_from_tag_children(&prod, &Tag::new("CEST")) {
                Some(ce[0].to_string())
            } else {
                None
            }
        };

        Ok(Produto {
            codigo,
            gtin,
            descricao,
            ncm,
            cest
        })
    }
}
