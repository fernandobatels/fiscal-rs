//! Destinarário da NF-e

use parsercher::dom::*;
use std::str::FromStr;
use super::endereco::*;

/// Destinarário base da NF-e
pub struct Destinatario {
    pub cnpj: String,
    pub razao_social: Option<String>,
    pub endereco: Option<Endereco>,
    pub ie: Option<String>,
    pub indicador_ie: IndicadorContribuicaoIe
}

impl Destinatario {
    /// Parse da seção <emit>
    pub(crate) fn parse(xml: &Dom) -> Result<Option<Destinatario>, String> {

        let mut t_dest = Dom::new(DomType::Tag);
        t_dest.set_tag(Tag::new("dest"));

        if let Some(dest) = parsercher::search_dom(&xml, &t_dest) {

            let cnpj = parsercher::search_text_from_tag_children(&dest, &Tag::new("CNPJ"))
                .ok_or("Tag <CNPJ> não encontrada na <dest>")?[0]
                .to_string();

            let razao_social = {
                if let Some(ra) = parsercher::search_text_from_tag_children(&dest, &Tag::new("xNome")) {
                    Some(ra[0].to_string())
                } else {
                    None
                }
            };

            let endereco = Endereco::parse(&xml, "enderDest")?;

            let ie = {
                if let Some(ie) = parsercher::search_text_from_tag_children(&dest, &Tag::new("IE")) {
                    Some(ie[0].to_string())
                } else {
                    None
                }
            };

            let indicador_ie = parsercher::search_text_from_tag_children(&dest, &Tag::new("indIEDest"))
                .ok_or("Tag <indIEDest> não encontrada na <dest>")?[0]
                .parse::<IndicadorContribuicaoIe>()?;

            Ok(Some(Destinatario {
                cnpj,
                razao_social,
                endereco,
                ie,
                indicador_ie
            }))
        } else {
            Ok(None)
        }
    }
}


/// Indicador da IE do destinatário
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum IndicadorContribuicaoIe {
    /// Contribuinte ICMS
    Contribuinte = 0,
    /// Contribuinte isento de Inscrição no cadastro de Contribuintes
    Isento = 1,
    /// Não Contribuinte, que pode ou não possuir Inscrição Estadual no Cadastro de Contribuintes do ICMS
    NaoContribuinte = 9,
}

impl FromStr for IndicadorContribuicaoIe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "9" => IndicadorContribuicaoIe::NaoContribuinte,
            "2" => IndicadorContribuicaoIe::Isento,
            _ => IndicadorContribuicaoIe::Contribuinte // 1
        })
    }
}
