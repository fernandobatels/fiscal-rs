//! NF-e - Representação da nota fiscal eletrônica

use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use std::convert::TryFrom;
use parsercher::{self, dom::*};
use super::ide::*;

/// Nota Fiscal Eletrônica
pub struct Nfe {
    pub versao: VersaoLayout,
    pub chave_acesso: String,
    pub ide: Identificacao
}

/// Versão do layout da NF-e
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum VersaoLayout {
    V4_00 = 4,
    Outra = -1
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

        let xml = parsercher::parse(s)
            .map_err(|e| e.to_string())?;

        // Saltamos direto para a tag <infNfe> já
        // que se não houver essa tag, de nada nos
        // adiantará a <NFe> ou <?xml>
        let infnfe = &parsercher::search_tag(&xml, &Tag::new("infNFe"))
            .ok_or("Tag <infNFe> não encontrada")?[0];

        let chave_acesso = infnfe.get_attr("Id")
            .ok_or("Atributo 'Id' não encontrado na tag <infNFe>")?
            .replace("NFe", "");

        let versao = infnfe.get_attr("versao")
            .ok_or("Atributo 'versao' não encontrado na tag <infNFe>")?
            .parse::<VersaoLayout>()?;

        let ide = Identificacao::parse(xml)?;

        Ok(Self {
            chave_acesso,
            versao,
            ide
        })
    }
}

impl FromStr for VersaoLayout {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "4.00" => VersaoLayout::V4_00,
            _ => VersaoLayout::Outra
        })
    }
}
