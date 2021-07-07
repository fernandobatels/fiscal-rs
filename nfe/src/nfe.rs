//! Interface para o modelo 55 da NF-e


use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::convert::{TryFrom, TryInto};
use super::ide::*;
use super::emit::*;
use super::dest::*;
use super::nfe_base::*;

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
            return Err(format!("Modelo do documento não suportador: {:?}", doc.ide.modelo));
        }

        let dest = doc.dest.ok_or("Destinatário não informado no documento")?;

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
        parse(s)?.try_into()
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
