//! Emitente da NF-e

use parsercher::dom::*;

/// Emitente da NF-e
pub struct Emitente {
    pub cnpj: String,
    pub razao_social: String,
    pub nome_fantasia: Option<String>,
    pub ie: String,
    pub iest: Option<u32>,
    pub endereco: Endereco,
}

pub struct Endereco {
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub codigo_municipio: u32,
    pub nome_municipio: String,
    pub sigla_uf: String,
    pub cep: u32,
    pub telefone: Option<String>
}

impl Emitente {

    /// Parse da seção <emit>
    pub(crate) fn parse(xml: &Dom) -> Result<Self, String> {

        let mut t_emit = Dom::new(DomType::Tag);
        t_emit.set_tag(Tag::new("emit"));

        let emit = parsercher::search_dom(&xml, &t_emit)
            .ok_or("Tag <emit> não encontrada")?;

        let cnpj = parsercher::search_text_from_tag_children(&emit, &Tag::new("CNPJ"))
            .ok_or("Tag <CNPJ> não encontrada na <emit>")?[0]
            .to_string();

        let razao_social = parsercher::search_text_from_tag_children(&emit, &Tag::new("xNome"))
            .ok_or("Tag <xNome> não encontrada na <emit>")?[0]
            .to_string();

        let nome_fantasia = {
            if let Some(fa) = parsercher::search_text_from_tag_children(&emit, &Tag::new("xFant")) {
                Some(fa[0].to_string())
            } else {
                None
            }
        };

        let ie = parsercher::search_text_from_tag_children(&emit, &Tag::new("IE"))
            .ok_or("Tag <IE> não encontrada na <emit>")?[0]
            .to_string();

        let iest = {
            if let Some(iest) = parsercher::search_text_from_tag_children(&emit, &Tag::new("IEST")) {
                Some(iest[0].parse::<u32>()
                     .map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let endereco = {

            let mut t_endr = Dom::new(DomType::Tag);
            t_endr.set_tag(Tag::new("enderEmit"));

            let endr = parsercher::search_dom(&xml, &t_endr)
                .ok_or("Tag <enderEmit> não encontrada")?;

            let logradouro = parsercher::search_text_from_tag_children(&endr, &Tag::new("xLgr"))
                .ok_or("Tag <xLgr> não encontrada na <enderEmit>")?[0]
                .to_string();

            let numero = parsercher::search_text_from_tag_children(&endr, &Tag::new("nro"))
                .ok_or("Tag <nro> não encontrada na <enderEmit>")?[0]
                .to_string();

            let complemento = {
                if let Some(comp) = parsercher::search_text_from_tag_children(&emit, &Tag::new("xCpl")) {
                    Some(comp[0].to_string())
                } else {
                    None
                }
            };

            let bairro = parsercher::search_text_from_tag_children(&endr, &Tag::new("xBairro"))
                .ok_or("Tag <xBairro> não encontrada na <enderEmit>")?[0]
                .to_string();

            let codigo_municipio = parsercher::search_text_from_tag_children(&endr, &Tag::new("cMun"))
                .ok_or("Tag <cMun> não encontrada na <enderEmit>")?[0]
                .parse::<u32>()
                .map_err(|e| e.to_string())?;

            let nome_municipio = parsercher::search_text_from_tag_children(&endr, &Tag::new("xMun"))
                .ok_or("Tag <xMun> não encontrada na <enderEmit>")?[0]
                .to_string();

            let sigla_uf = parsercher::search_text_from_tag_children(&endr, &Tag::new("UF"))
                .ok_or("Tag <UF> não encontrada na <enderEmit>")?[0]
                .to_string();

            let cep = parsercher::search_text_from_tag_children(&endr, &Tag::new("CEP"))
                .ok_or("Tag <CEP> não encontrada na <enderEmit>")?[0]
                .parse::<u32>()
                .map_err(|e| e.to_string())?;

            let telefone = {
                if let Some(fone) = parsercher::search_text_from_tag_children(&emit, &Tag::new("fone")) {
                    Some(fone[0].to_string())
                } else {
                    None
                }
            };

            Endereco {
                logradouro,
                numero,
                complemento,
                bairro,
                codigo_municipio,
                nome_municipio,
                sigla_uf,
                cep,
                telefone
            }
        };

        Ok(Emitente {
            cnpj,
            razao_social,
            nome_fantasia,
            ie,
            iest,
            endereco
        })
    }
}
