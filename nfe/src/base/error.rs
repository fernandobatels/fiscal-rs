//! Erros de parse, io...

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Falha na leitura do arquivo: {0}")]
    Io(std::io::Error),
    #[error("Falha no parse: {0}")]
    Serde(serde_xml_rs::Error),
}

impl From<serde_xml_rs::Error> for Error {
    fn from(se: serde_xml_rs::Error) -> Self {
        Self::Serde(se)
    }
}
