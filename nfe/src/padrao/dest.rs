//! Destinarário da NF-e

use parsercher::dom::*;

/// Destinarário da NF-e
pub struct Destinatario {
}

impl Destinatario {
    /// Parse da seção <emit>
    pub(crate) fn parse(xml: &Dom) -> Result<Option<Destinatario>, String> {

        let mut t_dest = Dom::new(DomType::Tag);
        t_dest.set_tag(Tag::new("dest"));

        if let Some(_dest) = parsercher::search_dom(&xml, &t_dest) {
            Ok(Some(Destinatario {
            }))
        } else {
            Ok(None)
        }
    }
}
