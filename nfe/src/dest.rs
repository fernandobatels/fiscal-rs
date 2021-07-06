//! Destinarário da NF-e

use parsercher::dom::*;

/// Destinarário da NF-e
pub struct Destinatario {
}

impl Destinatario {

    /// Parse da seção <emit>
    pub(crate) fn parse(xml: &Dom) -> Result<Self, String> {

        let mut t_dest = Dom::new(DomType::Tag);
        t_dest.set_tag(Tag::new("dest"));

        let dest = parsercher::search_dom(&xml, &t_dest)
            .ok_or("Tag <dest> não encontrada")?;

        Ok(Destinatario {
        })
    }
}
