//! NF-e - Representação da nota fiscal eletrônica

use parsercher::{self, dom::*};
use crate::comum::*;

pub(crate) mod dest;
pub use dest::*;

/// Base da Nota Fiscal Eletrônica
///
/// Representa o documento ainda sem a interface
/// do seu modelo(NF-e x NFC-e)
pub struct Nfe {
    pub versao: VersaoLayout,
    pub chave_acesso: String,
    pub ide: Identificacao,
    pub emit: Emitente,
    pub dest: Option<Destinatario>
}

/// Parse da NF-e, sem distinção dos modelos, a partir
/// de uma string
pub fn parse(s: &str) -> Result<Nfe, String> {

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

    let ide = Identificacao::parse(&xml)?;

    let emit = Emitente::parse(&xml)?;

    let dest = Destinatario::parse(&xml)?;

    Ok(Nfe {
        chave_acesso,
        versao,
        ide,
        emit,
        dest
    })
}
