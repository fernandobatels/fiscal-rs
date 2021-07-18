//! Detalhamento de produtos e serviços

use super::imposto::*;
use super::produto::*;
use parsercher::dom::*;

/// Item da nota
pub struct Item {
    pub numero: u8,
    pub produto: Produto,
    pub imposto: Imposto,
}

impl Item {
    /// Parse dos itens da nota
    pub(crate) fn parse(xml: &Dom) -> Result<Vec<Item>, String> {
        let det_itens =
            parsercher::search_tag(&xml, &Tag::new("det")).ok_or("Nenhuma tag <det> encontrada")?;

        let mut itens = vec![];

        for det in det_itens {
            let numero = det
                .get_attr("nItem")
                .ok_or("Atributo 'nItem' não encontrado na tag <det>")?
                .parse::<u8>()
                .map_err(|e| e.to_string())?;

            let mut t_det = Dom::new(DomType::Tag);
            t_det.set_tag(det);
            let det_dom = parsercher::search_dom(&xml, &t_det)
                .ok_or("Tag <det> não convertida para o tipo dom")?;

            let produto = Produto::parse(&det_dom)?;

            let imposto = Imposto::parse(&det_dom)?;

            itens.push(Item {
                numero,
                produto,
                imposto,
            });
        }

        Ok(itens)
    }
}
