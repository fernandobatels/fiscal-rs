//! Destinarário da NF-e

use parsercher::dom::*;
use crate::comum::*;

/// Destinarário base da NF-e
pub struct Destinatario {
    pub cnpj: String,
    pub razao_social: Option<String>,
    pub endereco: Option<Endereco>,
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

            Ok(Some(Destinatario {
                cnpj,
                razao_social,
                endereco
            }))
        } else {
            Ok(None)
        }
    }
}
