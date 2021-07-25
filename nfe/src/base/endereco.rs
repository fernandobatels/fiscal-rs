//! Endereço do emitente/destinatário da NF-e

use parsercher::dom::*;
use std::str::FromStr;
use serde::Deserialize;

/// Representação de um endereço usado na NFe
#[derive(Debug, Deserialize, PartialEq)]
pub struct Endereco {
    #[serde(rename="xLgr")]
    pub logradouro: String,
    #[serde(rename="nro")]
    pub numero: String,
    #[serde(rename="xCpl")]
    pub complemento: Option<String>,
    #[serde(rename="xBairro")]
    pub bairro: String,
    #[serde(rename="cMun")]
    pub codigo_municipio: u32,
    #[serde(rename="xMun")]
    pub nome_municipio: String,
    #[serde(rename="UF")]
    pub sigla_uf: String,
    #[serde(rename="CEP")]
    pub cep: u32,
    #[serde(rename="fone")]
    pub telefone: Option<String>,
}

impl Endereco {
    /// Parse da tag de endereço
    pub(crate) fn parse(xml: &Dom, tag: &str) -> Result<Option<Endereco>, String> {
        let mut t_endr = Dom::new(DomType::Tag);
        t_endr.set_tag(Tag::new(tag));

        if let Some(endr) = parsercher::search_dom(&xml, &t_endr) {
            let logradouro = parsercher::search_text_from_tag_children(&endr, &Tag::new("xLgr"))
                .ok_or(format!("Tag <xLgr> não encontrada na <{}>", tag))?[0]
                .to_string();

            let numero = parsercher::search_text_from_tag_children(&endr, &Tag::new("nro"))
                .ok_or(format!("Tag <nro> não encontrada na <{}>", tag))?[0]
                .to_string();

            let complemento = {
                if let Some(comp) =
                    parsercher::search_text_from_tag_children(&endr, &Tag::new("xCpl"))
                {
                    Some(comp[0].to_string())
                } else {
                    None
                }
            };

            let bairro = parsercher::search_text_from_tag_children(&endr, &Tag::new("xBairro"))
                .ok_or(format!("Tag <xBairro> não encontrada na <{}>", tag))?[0]
                .to_string();

            let codigo_municipio =
                parsercher::search_text_from_tag_children(&endr, &Tag::new("cMun"))
                    .ok_or(format!("Tag <cMun> não encontrada na <{}>", tag))?[0]
                    .parse::<u32>()
                    .map_err(|e| e.to_string())?;

            let nome_municipio =
                parsercher::search_text_from_tag_children(&endr, &Tag::new("xMun"))
                    .ok_or(format!("Tag <xMun> não encontrada na <{}>", tag))?[0]
                    .to_string();

            let sigla_uf = parsercher::search_text_from_tag_children(&endr, &Tag::new("UF"))
                .ok_or(format!("Tag <UF> não encontrada na <{}>", tag))?[0]
                .to_string();

            let cep = parsercher::search_text_from_tag_children(&endr, &Tag::new("CEP"))
                .ok_or(format!("Tag <CEP> não encontrada na <{}>", tag))?[0]
                .parse::<u32>()
                .map_err(|e| e.to_string())?;

            let telefone = {
                if let Some(fone) =
                    parsercher::search_text_from_tag_children(&endr, &Tag::new("fone"))
                {
                    Some(fone[0].to_string())
                } else {
                    None
                }
            };

            Ok(Some(Endereco {
                logradouro,
                numero,
                complemento,
                bairro,
                codigo_municipio,
                nome_municipio,
                sigla_uf,
                cep,
                telefone,
            }))
        } else {
            Ok(None)
        }
    }
}

impl FromStr for Endereco {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s)
            .map_err(|e| e.to_string())
    }
}
