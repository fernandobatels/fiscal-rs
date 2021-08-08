//! Erros referentes ao modelo 55

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Modelo do documento não suportado: {0:?}")]
    ModeloInvalido(crate::ModeloDocumentoFiscal),
    #[error("Destinatário inválido. {0}")]
    DestinatarioInvalido(String),
    #[error("{0}")]
    Base(crate::base::Error),
}

impl From<crate::base::Error> for Error {
    fn from(bs: crate::base::Error) -> Self {
        Self::Base(bs)
    }
}
