//! Emitente da NF-e

use parsercher::dom::*;

/// Emitente da NF-e
pub struct Emitente {
    pub cnpj: String,
    pub razao_social: String,
    pub nome_fantasia: Option<String>,
    pub ie: String,
    pub iest: Option<u32>,
}

impl Emitente {

    /// Parse da seção <emit>
    pub(crate) fn parse(xml: &Dom) -> Result<Self, String> {

        let mut t_ide = Dom::new(DomType::Tag);
        t_ide.set_tag(Tag::new("emit"));

        let emit = parsercher::search_dom(&xml, &t_ide)
            .ok_or("Tag <emit> não encontrada")?;

        let cnpj = parsercher::search_text_from_tag_children(&emit, &Tag::new("CNPJ"))
            .ok_or("Tag <CNPJ> não encontrada na <emit>")?[0]
            .to_string();

        let razao_social = parsercher::search_text_from_tag_children(&emit, &Tag::new("xNome"))
            .ok_or("Tag <xNome> não encontrada na <emit>")?[0]
            .to_string();

        let nome_fantasia = {
            if let Some(fa) = parsercher::search_text_from_tag_children(&emit, &Tag::new("xFant")) {
                Some(fa[0].to_string())
            } else {
                None
            }
        };

        let ie = parsercher::search_text_from_tag_children(&emit, &Tag::new("IE"))
            .ok_or("Tag <IE> não encontrada na <emit>")?[0]
            .to_string();

        let iest = {
            if let Some(iest) = parsercher::search_text_from_tag_children(&emit, &Tag::new("IEST")) {
                Some(iest[0].parse::<u32>()
                     .map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        Ok(Emitente {
            cnpj,
            razao_social,
            nome_fantasia,
            ie,
            iest
        })
    }
}
