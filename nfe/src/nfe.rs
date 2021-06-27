//! NF-e - Representação da nota fiscal eletrônica

use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::convert::TryFrom;

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

impl FromStr for Nfe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Nfe::parse(s)
    }
}

impl TryFrom<File> for Nfe {
    type Error = String;

    fn try_from(mut f: File) -> Result<Self, Self::Error> {

        let mut xml = String::new();
        f.read_to_string(&mut xml)
            .map_err(|e| e.to_string())?;

        Nfe::parse(&xml)
    }
}

impl Nfe {

    /// Parse da NF-e a partir de uma string
    pub fn parse(s: &str) -> Result<Self, String> {

        todo!()
    }
}
