//! Emitente da NF-e

use super::endereco::*;
use parsercher::dom::*;
use std::str::FromStr;
use serde::Deserialize;

/// Emitente da NF-e
#[derive(Debug, Deserialize, PartialEq)]
pub struct Emitente {
    #[serde(rename = "CNPJ")]
    pub cnpj: String,
    #[serde(rename = "xNome")]
    pub razao_social: String,
    #[serde(rename = "xFant")]
    pub nome_fantasia: Option<String>,
    #[serde(rename = "IE")]
    pub ie: String,
    #[serde(rename = "IEST")]
    pub iest: Option<u32>,
    #[serde(rename = "enderEmit")]
    pub endereco: Endereco,
}

impl Emitente {
    /// Parse da seção <emit>
    pub(crate) fn parse(xml: &Dom) -> Result<Emitente, String> {
        let mut t_emit = Dom::new(DomType::Tag);
        t_emit.set_tag(Tag::new("emit"));

        let emit = parsercher::search_dom(&xml, &t_emit).ok_or("Tag <emit> não encontrada")?;

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
            if let Some(iest) = parsercher::search_text_from_tag_children(&emit, &Tag::new("IEST"))
            {
                Some(iest[0].parse::<u32>().map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let endereco = Endereco::parse(&xml, "enderEmit")?
            .ok_or("Tag <enderEmit> não encontrada na <emit>")?;

        Ok(Emitente {
            cnpj,
            razao_social,
            nome_fantasia,
            ie,
            iest,
            endereco,
        })
    }
}

impl FromStr for Emitente {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s)
            .map_err(|e| e.to_string())
    }
}
