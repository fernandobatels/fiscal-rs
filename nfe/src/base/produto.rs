//! Produtos

use std::str::FromStr;
use parsercher::dom::*;

/// Produto do item da nota
pub struct Produto {
    pub codigo: String,
    /// GTIN (Global Trade Item Number) do produto, antigo código EAN ou código de barras
    pub gtin: Option<String>,
    pub descricao: String,
    /// NCM - Nomenclatura Comum do Mercosul
    pub ncm: String,
    /// CEST - Código Especificador da Substituição Tributária
    pub cest: Option<String>,
    /// Indicador de Produção em escala relevante
    pub escala_relevante: Option<EscalaRelevante>,
    /// CNPJ do Fabricante da Mercadoria
    pub fabricante_cnpj: Option<String>,
    /// Código de Benefício Fiscal na UF aplicado ao item
    pub codigo_beneficio_fiscal: Option<String>,
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

        let escala_relevante = {
            if let Some(er) = parsercher::search_text_from_tag_children(&prod, &Tag::new("indEscala")) {
                Some(er[0].parse::<EscalaRelevante>()?)
            } else {
                None
            }
        };

        let fabricante_cnpj = {
            if let Some(fa) = parsercher::search_text_from_tag_children(&prod, &Tag::new("CNPJFab")) {
                Some(fa[0].to_string())
            } else {
                None
            }
        };

        let codigo_beneficio_fiscal = {
            if let Some(cb) = parsercher::search_text_from_tag_children(&prod, &Tag::new("cBenef")) {
                Some(cb[0].to_string())
            } else {
                None
            }
        };

        Ok(Produto {
            codigo,
            gtin,
            descricao,
            ncm,
            cest,
            escala_relevante,
            fabricante_cnpj,
            codigo_beneficio_fiscal
        })
    }
}

/// Indicador de Produção em escala relevante
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum EscalaRelevante {
    Sim = 1,
    Nao = 2
}

impl FromStr for EscalaRelevante {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().trim() {
            "s" => EscalaRelevante::Nao, // S
            _ => EscalaRelevante::Nao // N
        })
    }
}
