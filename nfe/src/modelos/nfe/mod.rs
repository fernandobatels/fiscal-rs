//! Modelo 55 da NF-e

pub use crate::base::emit::*;
pub use crate::base::endereco::*;
pub use crate::base::ide::*;
pub use crate::base::item::*;
pub use crate::base::totais::*;
pub use crate::base::transporte::*;
use crate::base::Nfe as NfeBase;
pub use crate::base::VersaoLayout;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::str::FromStr;

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
    pub dest: Destinatario,
    pub itens: Vec<Item>,
    pub totais: Totalizacao,
    pub transporte: Transporte,
    /// Informações complementares de interesse do contribuinte
    pub informacao_complementar: Option<String>,
}

impl TryFrom<NfeBase> for Nfe {
    type Error = String;

    fn try_from(doc: NfeBase) -> Result<Self, Self::Error> {
        if doc.ide.modelo != ModeloDocumentoFiscal::Nfe {
            return Err(format!(
                "Modelo do documento não suportado: {:?}",
                doc.ide.modelo
            ));
        }

        let dest = doc
            .dest
            .ok_or("Destinatário não informado no documento")?
            .try_into()?;

        Ok(Self {
            versao: doc.versao,
            chave_acesso: doc.chave_acesso,
            ide: doc.ide,
            emit: doc.emit,
            dest,
            itens: doc.itens,
            totais: doc.totais,
            transporte: doc.transporte,
            informacao_complementar: doc.informacao_complementar,
        })
    }
}

impl FromStr for Nfe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<NfeBase>()?.try_into()
    }
}

impl TryFrom<File> for Nfe {
    type Error = String;

    fn try_from(f: File) -> Result<Self, Self::Error> {
        NfeBase::try_from(f)?.try_into()
    }
}
