//! Identificação da NF-e

use std::str::FromStr;
use parsercher::dom::*;

/// Identificação da NF-e
pub struct Identificacao {
    pub codigo_uf: u8,
    pub codigo_chave: u32,
    pub numero: u32,
    pub serie: u16,
    pub natureza_operacao: String,
    pub modelo: ModeloDocumentoFiscal
}

/// Modelo do documento fiscal: NF-e ou NFC-e
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ModeloDocumentoFiscal {
    Nfe = 55,
    Nfce = 65,
    Desconhecido = -1
}

impl Identificacao {

    /// Parse da seção <ide>
    pub(crate) fn parse(xml: Dom) -> Result<Self, String> {

        let mut t_ide = Dom::new(DomType::Tag);
        t_ide.set_tag(Tag::new("ide"));

        let ide = parsercher::search_dom(&xml, &t_ide)
            .ok_or("Tag <ide> não encontrada")?;

        let codigo_uf = parsercher::search_text_from_tag_children(&ide, &Tag::new("cUF"))
            .ok_or("Tag <cUF> não encontrada na <ide>")?[0]
            .parse::<u8>()
            .map_err(|e| e.to_string())?;

        let codigo_chave = parsercher::search_text_from_tag_children(&ide, &Tag::new("cNF"))
            .ok_or("Tag <cNF> não encontrada na <ide>")?[0]
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        let natureza_operacao = parsercher::search_text_from_tag_children(&ide, &Tag::new("natOp"))
            .ok_or("Tag <natOp> não encontrada na <ide>")?[0]
            .to_string();

        let serie = parsercher::search_text_from_tag_children(&ide, &Tag::new("serie"))
            .ok_or("Tag <serie> não encontrada na <ide>")?[0]
            .parse::<u16>()
            .map_err(|e| e.to_string())?;

        let numero = parsercher::search_text_from_tag_children(&ide, &Tag::new("nNF"))
            .ok_or("Tag <nNF> não encontrada na <ide>")?[0]
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        let modelo = parsercher::search_text_from_tag_children(&ide, &Tag::new("mod"))
            .ok_or("Tag <mod> não encontrada na <ide>")?[0]
            .parse::<ModeloDocumentoFiscal>()?;

        Ok(Identificacao {
            codigo_uf,
            codigo_chave,
            serie,
            numero,
            natureza_operacao,
            modelo
        })
    }
}

impl FromStr for ModeloDocumentoFiscal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "55" => ModeloDocumentoFiscal::Nfe,
            "65" => ModeloDocumentoFiscal::Nfce,
            _ => ModeloDocumentoFiscal::Desconhecido
        })
    }
}
