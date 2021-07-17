//! Endereço do emitente/destinatário da NF-e

use parsercher::dom::*;

/// Representação de um endereço usado na NFe
pub struct Endereco {
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub codigo_municipio: u32,
    pub nome_municipio: String,
    pub sigla_uf: String,
    pub cep: u32,
    pub telefone: Option<String>,
}

impl Endereco {
    /// Parse da tag de endereço
    pub(crate) fn parse(xml: &Dom, tag: &str) -> Result<Option<Endereco>, String> {
        let mut t_endr = Dom::new(DomType::Tag);
        t_endr.set_tag(Tag::new(tag));

        if let Some(endr) = parsercher::search_dom(&xml, &t_endr) {
            let logradouro = parsercher::search_text_from_tag_children(&endr, &Tag::new("xLgr"))
                .ok_or(format!("Tag <xLgr> não encontrada na <{}>", tag))?[0]
                .to_string();

            let numero = parsercher::search_text_from_tag_children(&endr, &Tag::new("nro"))
                .ok_or(format!("Tag <nro> não encontrada na <{}>", tag))?[0]
                .to_string();

            let complemento = {
                if let Some(comp) =
                    parsercher::search_text_from_tag_children(&endr, &Tag::new("xCpl"))
                {
                    Some(comp[0].to_string())
                } else {
                    None
                }
            };

            let bairro = parsercher::search_text_from_tag_children(&endr, &Tag::new("xBairro"))
                .ok_or(format!("Tag <xBairro> não encontrada na <{}>", tag))?[0]
                .to_string();

            let codigo_municipio =
                parsercher::search_text_from_tag_children(&endr, &Tag::new("cMun"))
                    .ok_or(format!("Tag <cMun> não encontrada na <{}>", tag))?[0]
                    .parse::<u32>()
                    .map_err(|e| e.to_string())?;

            let nome_municipio =
                parsercher::search_text_from_tag_children(&endr, &Tag::new("xMun"))
                    .ok_or(format!("Tag <xMun> não encontrada na <{}>", tag))?[0]
                    .to_string();

            let sigla_uf = parsercher::search_text_from_tag_children(&endr, &Tag::new("UF"))
                .ok_or(format!("Tag <UF> não encontrada na <{}>", tag))?[0]
                .to_string();

            let cep = parsercher::search_text_from_tag_children(&endr, &Tag::new("CEP"))
                .ok_or(format!("Tag <CEP> não encontrada na <{}>", tag))?[0]
                .parse::<u32>()
                .map_err(|e| e.to_string())?;

            let telefone = {
                if let Some(fone) =
                    parsercher::search_text_from_tag_children(&endr, &Tag::new("fone"))
                {
                    Some(fone[0].to_string())
                } else {
                    None
                }
            };

            Ok(Some(Endereco {
                logradouro,
                numero,
                complemento,
                bairro,
                codigo_municipio,
                nome_municipio,
                sigla_uf,
                cep,
                telefone,
            }))
        } else {
            Ok(None)
        }
    }
}
