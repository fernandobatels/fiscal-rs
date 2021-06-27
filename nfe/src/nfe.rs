//! NF-e - Representação da nota fiscal eletrônica

use std::io::{BufRead, BufReader};
use std::str::FromStr;
use xml::{*, reader::XmlEvent};

/// Nota Fiscal Eletrônica
pub struct Nfe {
    pub versao: VersaoLayout,
    pub chave_acesso: String
}

/// Versão do layout da NF-e
#[derive(Debug)]
pub enum VersaoLayout {
    V4_00,
    Outra
}

/// Parse da NF-e a partir da string
impl FromStr for Nfe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Nfe::parse(BufReader::new(s.as_bytes()))
    }
}

impl Nfe {

    /// Parse da NF-e a partir de um reader
    pub fn parse<B>(buffer: B) -> Result<Self, String>
    where
        B: BufRead {
        let mut parser = EventReader::new(buffer);

        match parser.next() {
            Ok(XmlEvent::StartDocument { .. }) => Ok(()),
            _ => Err("Tag inicial do XML não encontrada".to_string())
        }.map_err(|e| e.to_string())?;

        todo!()
    }
}
