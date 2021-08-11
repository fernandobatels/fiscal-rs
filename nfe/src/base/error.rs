//! Erros de parse, io...

use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display(fmt = "Falha na leitura do arquivo: {}", _0)]
    Io(std::io::Error),
    #[display(fmt = "Falha no parse: {}", _0)]
    Serde(serde_xml_rs::Error),
}
