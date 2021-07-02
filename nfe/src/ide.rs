//! Identificação da NF-e

use std::str::FromStr;
use parsercher::dom::*;
use chrono::prelude::*;

/// Identificação da NF-e
pub struct Identificacao {
    pub codigo_uf: u8,
    pub codigo_chave: u32,
    pub numero: u32,
    pub serie: u16,
    pub natureza_operacao: String,
    pub modelo: ModeloDocumentoFiscal,
    pub emissao: DateTime<Utc>,
    /// Horário de saída ou da entrada do produto
    pub operacao: Option<DateTime<Utc>>,
    pub tipo_operacao: TipoOperacao,
    pub destino_operacao: DestinoOperacao
}

/// Modelo do documento fiscal: NF-e ou NFC-e
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ModeloDocumentoFiscal {
    Nfe = 55,
    Nfce = 65
}

/// Tipo de operação da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoOperacao {
    Entrada = 0,
    Saida = 1
}

/// Destino da operação da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DestinoOperacao {
    OperacaoInterna = 0,
    OperacaoInterestadual = 1,
    OperacaoComExterior = 2
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

        let emissao = parsercher::search_text_from_tag_children(&ide, &Tag::new("dhEmi"))
            .ok_or("Tag <dhEmi> não encontrada na <ide>")?[0]
            .parse::<DateTime<Utc>>()
            .map_err(|e| e.to_string())?;

        let operacao = {
            if let Some(dt) = parsercher::search_text_from_tag_children(&ide, &Tag::new("dhSaiEnt")) {
                Some(dt[0].parse::<DateTime<Utc>>()
                    .map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let tipo_operacao = parsercher::search_text_from_tag_children(&ide, &Tag::new("tpNF"))
            .ok_or("Tag <tpNF> não encontrada na <ide>")?[0]
            .parse::<TipoOperacao>()?;

        let destino_operacao = parsercher::search_text_from_tag_children(&ide, &Tag::new("idDest"))
            .ok_or("Tag <idDest> não encontrada na <ide>")?[0]
            .parse::<DestinoOperacao>()?;

        Ok(Identificacao {
            codigo_uf,
            codigo_chave,
            serie,
            numero,
            natureza_operacao,
            modelo,
            emissao,
            operacao,
            tipo_operacao,
            destino_operacao
        })
    }
}

impl FromStr for ModeloDocumentoFiscal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "65" => ModeloDocumentoFiscal::Nfce,
            _ => ModeloDocumentoFiscal::Nfe // 55
        })
    }
}

impl FromStr for TipoOperacao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => TipoOperacao::Saida,
            _ => TipoOperacao::Entrada
        })
    }
}

impl FromStr for DestinoOperacao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "3" => DestinoOperacao::OperacaoComExterior,
            "2" => DestinoOperacao::OperacaoInterestadual,
            _ => DestinoOperacao::OperacaoInterna // 1
        })
    }
}
