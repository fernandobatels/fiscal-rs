//! Identificação da NF-e

use std::str::FromStr;
use parsercher::dom::*;
use super::operacao::*;
use super::emissao::*;

/// Identificação da NF-e
pub struct Identificacao {
    pub codigo_uf: u8,
    pub chave: ComposicaoChaveAcesso,
    pub numero: u32,
    pub serie: u16,
    pub modelo: ModeloDocumentoFiscal,
    pub emissao: Emissao,
    pub operacao: Operacao,
    pub codigo_municipio: u32,
    pub formato_danfe: FormatoImpressaoDanfe,
    pub ambiente: TipoAmbiente
}

impl Identificacao {
    /// Parse da seção <ide>
    pub fn parse(xml: &Dom) -> Result<Identificacao, String> {

        let mut t_ide = Dom::new(DomType::Tag);
        t_ide.set_tag(Tag::new("ide"));

        let ide = parsercher::search_dom(&xml, &t_ide)
            .ok_or("Tag <ide> não encontrada")?;

        let codigo_uf = parsercher::search_text_from_tag_children(&ide, &Tag::new("cUF"))
            .ok_or("Tag <cUF> não encontrada na <ide>")?[0]
            .parse::<u8>()
            .map_err(|e| e.to_string())?;

        let chave = {
            let codigo = parsercher::search_text_from_tag_children(&ide, &Tag::new("cNF"))
                .ok_or("Tag <cNF> não encontrada na <ide>")?[0]
                .parse::<u32>()
                .map_err(|e| e.to_string())?;

            let digito_verificador = parsercher::search_text_from_tag_children(&ide, &Tag::new("cDV"))
                .ok_or("Tag <cDV> não encontrada na <ide>")?[0]
                .parse::<u8>()
                .map_err(|e| e.to_string())?;

            ComposicaoChaveAcesso {
                codigo,
                digito_verificador
            }
        };

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

        let emissao = Emissao::parse(&ide)?;

        let operacao = Operacao::parse(&ide)?;

        let codigo_municipio = parsercher::search_text_from_tag_children(&ide, &Tag::new("cMunFG"))
            .ok_or("Tag <cMunFG> não encontrada na <ide>")?[0]
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        let formato_danfe = parsercher::search_text_from_tag_children(&ide, &Tag::new("tpImp"))
            .ok_or("Tag <tpImp> não encontrada na <ide>")?[0]
            .parse::<FormatoImpressaoDanfe>()?;

        let ambiente = parsercher::search_text_from_tag_children(&ide, &Tag::new("tpAmb"))
            .ok_or("Tag <tpAmb> não encontrada na <ide>")?[0]
            .parse::<TipoAmbiente>()?;

        Ok(Identificacao {
            codigo_uf,
            chave,
            serie,
            numero,
            modelo,
            emissao,
            operacao,
            codigo_municipio,
            formato_danfe,
            ambiente
        })
    }
}


/// Modelo do documento fiscal: NF-e ou NFC-e
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ModeloDocumentoFiscal {
    Nfe = 55,
    Nfce = 65
}

/// Formato de impressão do DANFE
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FormatoImpressaoDanfe {
    SemGeracao = 0,
    NormalRetrato = 1,
    NormalPaisagem = 2,
    Simplificado = 3,
    Nfce = 4,
    NfceMensagemEletronica = 5
}

/// Tipo do ambiente da NF
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoAmbiente {
    Producao = 1,
    Homologacao = 2,
}

/// Dados referentes a regeração da chave de acesso
pub struct ComposicaoChaveAcesso {
    pub codigo: u32,
    pub digito_verificador: u8
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

impl FromStr for FormatoImpressaoDanfe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "5" => FormatoImpressaoDanfe::NfceMensagemEletronica,
            "4" => FormatoImpressaoDanfe::Nfce,
            "3" => FormatoImpressaoDanfe::Simplificado,
            "2" => FormatoImpressaoDanfe::NormalPaisagem,
            "1" => FormatoImpressaoDanfe::NormalRetrato,
            _ => FormatoImpressaoDanfe::SemGeracao // 0
        })
    }
}

impl FromStr for TipoAmbiente {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => TipoAmbiente::Producao,
            _ => TipoAmbiente::Homologacao // 2
        })
    }
}
