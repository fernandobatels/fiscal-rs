//! Informações sobre o transporte da nota

use parsercher::dom::*;
use std::str::FromStr;
use serde::Deserialize;
use serde_repr::Deserialize_repr;


/// Transporte da nota
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "transp")]
pub struct Transporte {
    /// Modalidade do frete
    #[serde(rename = "modFrete")]
    pub modalidade: ModalidadeFrete,
}

impl Transporte {
    /// Parse da seção <total>
    pub(crate) fn parse(xml: &Dom) -> Result<Transporte, String> {
        let mut t_transp = Dom::new(DomType::Tag);
        t_transp.set_tag(Tag::new("transp"));
        let transp =
            parsercher::search_dom(&xml, &t_transp).ok_or("Tag <transp> não encontrada")?;

        let modalidade = parsercher::search_text_from_tag_children(&transp, &Tag::new("modFrete"))
            .ok_or("Tag <mdoFrete> não encontrada na <transp>")?[0]
            .parse::<ModalidadeFrete>()?;

        Ok(Transporte { modalidade })
    }
}

impl FromStr for Transporte {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s)
            .map_err(|e| e.to_string())
    }
}

/// Modalidade do frete
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum ModalidadeFrete {
    /// CIF - Contratação do frete por conta do remetente
    ContratacaoPorContaRemetente = 0,
    /// FOB - Contratação do frete por conta do destinarário
    ContratacaoPorContaDestinatario = 1,
    /// Contratação do frete por conta de terceiros
    ContratacaoPorContaTerceiros = 2,
    /// Transporte próprio por conta do remetente
    TransportePorContaRemetente = 3,
    /// Transporte próprio por conta do destinatário
    TransportePorContaDestinatario = 4,
    /// Sem ocorrência de transporte
    SemTransporte = 9,
}

impl FromStr for ModalidadeFrete {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "9" => ModalidadeFrete::SemTransporte,
            "4" => ModalidadeFrete::TransportePorContaDestinatario,
            "3" => ModalidadeFrete::TransportePorContaRemetente,
            "2" => ModalidadeFrete::ContratacaoPorContaTerceiros,
            "1" => ModalidadeFrete::ContratacaoPorContaDestinatario,
            "0" => ModalidadeFrete::ContratacaoPorContaRemetente,
            _ => unreachable!()
        })
    }
}
