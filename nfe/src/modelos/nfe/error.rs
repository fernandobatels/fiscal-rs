//! Erros referentes ao modelo 55

use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display(fmt = "Modelo do documento não suportado: {:?}", _0)]
    ModeloInvalido(#[error(not(source))] crate::ModeloDocumentoFiscal),
    #[display(fmt = "Destinatário inválido. {}", _0)]
    DestinatarioInvalido(#[error(not(source))] String),
    Base(crate::base::Error),
}
