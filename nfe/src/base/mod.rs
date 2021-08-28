//! Base da NF-e
//!
//! Tipos e estruturas para tratamento da NF-e sem
//! distinção dos modelos.

use serde::{Deserialize, Deserializer};
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
pub mod dest;
pub mod emit;
pub mod endereco;
mod error;
pub mod ide;
pub mod item;
pub mod totais;
pub mod transporte;
use dest::Destinatario;
use emit::Emitente;
pub use error::Error;
use ide::Identificacao;
use item::Item;
use totais::Totalizacao;
use transporte::Transporte;

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

/// Versão do layout da NF-e
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize)]
pub enum VersaoLayout {
    #[serde(rename = "4.00")]
    V4_00 = 4,
}

impl FromStr for Nfe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl TryFrom<File> for Nfe {
    type Error = Error;

    fn try_from(mut f: File) -> Result<Self, Self::Error> {
        let mut xml = String::new();
        f.read_to_string(&mut xml).map_err(|e| Error::Io(e))?;

        xml.parse::<Nfe>()
    }
}

impl<'de> Deserialize<'de> for Nfe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "NFe")]
        struct NfeRoot {
            #[serde(rename = "infNFe")]
            pub inf: NfeHelper,
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
                None => None,
            },
        })
    }
}
