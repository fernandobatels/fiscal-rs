//! Base da NF-e
//!
//! Tipos e estruturas para tratamento da NF-e sem
//! distinção dos modelos.

use parsercher::{self, dom::*};
use serde::{Deserialize, Deserializer};
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
pub mod dest;
pub mod emit;
pub mod endereco;
pub mod ide;
pub mod item;
pub mod totais;
pub mod transporte;
pub mod versao;
use dest::Destinatario;
use emit::Emitente;
use ide::Identificacao;
use item::Item;
use totais::Totalizacao;
use transporte::Transporte;
use versao::VersaoLayout;

/// Base da Nota Fiscal Eletrônica
///
/// Representa o documento ainda sem a interface
/// do seu modelo(NF-e x NFC-e)
#[derive(Debug, PartialEq)]
pub struct Nfe {
    pub versao: VersaoLayout,
    pub chave_acesso: String,
    pub ide: Identificacao,
    pub emit: Emitente,
    pub dest: Option<Destinatario>,
    pub itens: Vec<Item>,
    pub totais: Totalizacao,
    pub transporte: Transporte,
    /// Informações complementares de interesse do contribuinte
    pub informacao_complementar: Option<String>,
}

impl Nfe {
    /// Parse da NF-e, sem distinção dos modelos, a partir
    /// de uma string
    pub(crate) fn parse(s: &str) -> Result<Nfe, String> {
        let xml = parsercher::parse(s).map_err(|e| e.to_string())?;

        // Saltamos direto para a tag <infNfe> já
        // que se não houver essa tag, de nada nos
        // adiantará a <NFe> ou <?xml>
        let infnfe = &parsercher::search_tag(&xml, &Tag::new("infNFe"))
            .ok_or("Tag <infNFe> não encontrada")?[0];

        let chave_acesso = infnfe
            .get_attr("Id")
            .ok_or("Atributo 'Id' não encontrado na tag <infNFe>")?
            .replace("NFe", "");

        let versao = infnfe
            .get_attr("versao")
            .ok_or("Atributo 'versao' não encontrado na tag <infNFe>")?
            .parse::<VersaoLayout>()?;

        let ide = Identificacao::parse(&xml)?;

        let emit = Emitente::parse(&xml)?;

        let dest = Destinatario::parse(&xml)?;

        let itens = Item::parse(&xml)?;

        let totais = Totalizacao::parse(&xml)?;

        let transporte = Transporte::parse(&xml)?;

        let informacao_complementar = {
            let mut t_inf_adic = Dom::new(DomType::Tag);
            t_inf_adic.set_tag(Tag::new("infAdic"));

            if let Some(inf_adic) = parsercher::search_dom(&xml, &t_inf_adic) {
                if let Some(cpl) =
                    parsercher::search_text_from_tag_children(&inf_adic, &Tag::new("infCpl"))
                {
                    Some(cpl[0].to_string())
                } else {
                    None
                }
            } else {
                None
            }
        };

        Ok(Nfe {
            chave_acesso,
            versao,
            ide,
            emit,
            dest,
            itens,
            totais,
            transporte,
            informacao_complementar,
        })
    }
}

impl FromStr for Nfe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s)
            .map_err(|e| e.to_string())
    }
}

impl TryFrom<File> for Nfe {
    type Error = String;

    fn try_from(mut f: File) -> Result<Self, Self::Error> {
        let mut xml = String::new();
        f.read_to_string(&mut xml).map_err(|e| e.to_string())?;

        xml.parse::<Nfe>()
    }
}


impl<'de> Deserialize<'de> for Nfe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(rename = "NFe")]
        struct NfeRoot {
            #[serde(rename = "infNFe")]
            pub inf: NfeHelper
        }

        #[derive(Deserialize)]
        struct InfAdd {
            #[serde(rename = "infCpl")]
            pub informacao_complementar: Option<String>,
        }

        #[derive(Deserialize)]
        struct NfeHelper {
            #[serde(rename = "versao")]
            pub versao: VersaoLayout,
            #[serde(rename = "Id")]
            pub chave_acesso: String,
            #[serde(rename = "ide")]
            pub ide: Identificacao,
            #[serde(rename = "emit")]
            pub emit: Emitente,
            #[serde(rename = "dest")]
            pub dest: Option<Destinatario>,
            #[serde(rename = "det")]
            pub itens: Vec<Item>,
            #[serde(rename = "total")]
            pub totais: Totalizacao,
            #[serde(rename = "transp")]
            pub transporte: Transporte,
            #[serde(rename = "infAdic")]
            pub add: Option<InfAdd>,
        }

        let nfe = NfeRoot::deserialize(deserializer)?;

        Ok(Self {
            versao: nfe.inf.versao,
            chave_acesso: nfe.inf.chave_acesso.replace("NFe", ""),
            ide: nfe.inf.ide,
            emit: nfe.inf.emit,
            dest: nfe.inf.dest,
            itens: nfe.inf.itens,
            totais: nfe.inf.totais,
            transporte: nfe.inf.transporte,
            informacao_complementar: match nfe.inf.add {
                Some(add) => add.informacao_complementar,
                None => None
            },
        })
    }
}
