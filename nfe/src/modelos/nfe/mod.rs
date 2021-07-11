//! Modelo 55 da NF-e

use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::convert::{TryFrom, TryInto};
pub use crate::base::versao::*;
pub use crate::base::ide::*;
pub use crate::base::emit::*;
pub use crate::base::endereco::*;
pub use crate::base::operacao::*;
pub use crate::base::emissao::*;
use crate::base::{self as nfe_base, Nfe as NfeBase};

mod dest;
pub use dest::*;

/// Nota Fiscal Eletrônica
///
/// Apenas o modelo 55 é suportado
pub struct Nfe {
    pub versao: VersaoLayout,
    pub chave_acesso: String,
    pub ide: Identificacao,
    pub emit: Emitente,
    pub dest: Destinatario
}

impl TryFrom<NfeBase> for Nfe {
    type Error = String;

    fn try_from(doc: NfeBase) -> Result<Self, Self::Error> {

        if doc.ide.modelo != ModeloDocumentoFiscal::Nfe {
            return Err(format!("Modelo do documento não suportado: {:?}", doc.ide.modelo));
        }

        let dest = doc.dest.ok_or("Destinatário não informado no documento")?.try_into()?;

        Ok(Self {
            versao: doc.versao,
            chave_acesso: doc.chave_acesso,
            ide: doc.ide,
            emit: doc.emit,
            dest: dest,
        })
    }
}

impl FromStr for Nfe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        nfe_base::parse(s)?.try_into()
    }
}

impl TryFrom<File> for Nfe {
    type Error = String;

    fn try_from(mut f: File) -> Result<Self, Self::Error> {

        let mut xml = String::new();
        f.read_to_string(&mut xml)
            .map_err(|e| e.to_string())?;

        xml.parse::<Nfe>()
    }
}
