//! Base da NF-e
//!
//! Tipos e estruturas para tratamento da NF-e sem
//! distinção dos modelos.

use parsercher::{self, dom::*};
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
pub mod dest;
pub mod emissao;
pub mod emit;
pub mod endereco;
pub mod ide;
pub mod imposto;
pub mod item;
pub mod operacao;
pub mod produto;
pub mod versao;
use dest::Destinatario;
use emit::Emitente;
use ide::Identificacao;
use item::Item;
use versao::VersaoLayout;

/// Base da Nota Fiscal Eletrônica
///
/// Representa o documento ainda sem a interface
/// do seu modelo(NF-e x NFC-e)
pub struct Nfe {
    pub versao: VersaoLayout,
    pub chave_acesso: String,
    pub ide: Identificacao,
    pub emit: Emitente,
    pub dest: Option<Destinatario>,
    pub itens: Vec<Item>,
}

impl Nfe {
    /// Parse da NF-e, sem distinção dos modelos, a partir
    /// de uma string
    pub(crate) fn parse(s: &str) -> Result<Nfe, String> {
        let xml = parsercher::parse(s).map_err(|e| e.to_string())?;

        // Saltamos direto para a tag <infNfe> já
        // que se não houver essa tag, de nada nos
        // adiantará a <NFe> ou <?xml>
        let infnfe = &parsercher::search_tag(&xml, &Tag::new("infNFe"))
            .ok_or("Tag <infNFe> não encontrada")?[0];

        let chave_acesso = infnfe
            .get_attr("Id")
            .ok_or("Atributo 'Id' não encontrado na tag <infNFe>")?
            .replace("NFe", "");

        let versao = infnfe
            .get_attr("versao")
            .ok_or("Atributo 'versao' não encontrado na tag <infNFe>")?
            .parse::<VersaoLayout>()?;

        let ide = Identificacao::parse(&xml)?;

        let emit = Emitente::parse(&xml)?;

        let dest = Destinatario::parse(&xml)?;

        let itens = Item::parse(&xml)?;

        Ok(Nfe {
            chave_acesso,
            versao,
            ide,
            emit,
            dest,
            itens,
        })
    }
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
        f.read_to_string(&mut xml).map_err(|e| e.to_string())?;

        xml.parse::<Nfe>()
    }
}
