/// Grupos de COFINS
use parsercher::dom::*;
use serde::Deserialize;

/// COFINS
#[derive(Debug, PartialEq, Deserialize)]
pub enum GrupoCofins {
    /// Outras Operações
    #[serde(rename = "COFINSOutr")]
    CofinsOutr(GrupoCofinsOutr),
    /// Não Tributado
    #[serde(rename = "COFINSNT")]
    CofinsNt(GrupoCofinsNt),
    /// Tributado pela alíquota
    #[serde(rename = "COFINSAliq")]
    CofinsAliq(GrupoCofinsAliq),
}

impl GrupoCofins {
    /// Parse dos tipos de COFINS do item
    pub(crate) fn parse(imposto: &Dom) -> Result<Option<GrupoCofins>, String> {
        let mut t_cofins = Dom::new(DomType::Tag);

        t_cofins.set_tag(Tag::new("COFINSOutr"));
        if let Some(cofins) = parsercher::search_dom(&imposto, &t_cofins) {
            return Ok(Some(GrupoCofins::CofinsOutr(GrupoCofinsOutr::parse(
                &cofins,
            )?)));
        }

        t_cofins.set_tag(Tag::new("COFINSNT"));
        if let Some(cofins) = parsercher::search_dom(&imposto, &t_cofins) {
            return Ok(Some(GrupoCofins::CofinsNt(GrupoCofinsNt::parse(&cofins)?)));
        }

        t_cofins.set_tag(Tag::new("COFINSAliq"));
        if let Some(cofins) = parsercher::search_dom(&imposto, &t_cofins) {
            return Ok(Some(GrupoCofins::CofinsAliq(GrupoCofinsAliq::parse(
                &cofins,
            )?)));
        }

        Ok(None)
    }
}

/// Grupo COFINS Outr - Outras Operações
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoCofinsOutr {
    /// CST - Código de Situação Tributária do COFINS
    #[serde(rename = "CST")]
    pub codigo_situacao: String,
    /// Valor da base de cálculo do COFINS
    #[serde(rename = "vBC")]
    pub valor_base_calculo: f32,
    /// Alíquota do COFINS(%)
    #[serde(rename = "pCOFINS")]
    pub aliquota: f32,
}

impl GrupoCofinsOutr {
    /// Parse do COFINSOutr
    pub(crate) fn parse(cofins: &Dom) -> Result<GrupoCofinsOutr, String> {
        let aliquota = parsercher::search_text_from_tag_children(&cofins, &Tag::new("pCOFINS"))
            .ok_or("Tag <pCOFINS> não encontrada na <COFINSOutr>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_base_calculo =
            parsercher::search_text_from_tag_children(&cofins, &Tag::new("vBC"))
                .ok_or("Tag <vBC> não encontrada na <COFINSOutr>")?[0]
                .parse::<f32>()
                .map_err(|e| e.to_string())?;

        let codigo_situacao = parsercher::search_text_from_tag_children(&cofins, &Tag::new("CST"))
            .ok_or("Tag <CST> não encontrada na <COFINSOutr>")?[0]
            .to_string();

        Ok(GrupoCofinsOutr {
            aliquota,
            valor_base_calculo,
            codigo_situacao,
        })
    }
}

/// Grupo COFINS NT - COFINS não tributado
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoCofinsNt {
    /// CST - Código de Situação Tributária do COFINS
    #[serde(rename = "CST")]
    pub codigo_situacao: String,
}

impl GrupoCofinsNt {
    /// Parse do COFINSNT
    pub(crate) fn parse(cofins: &Dom) -> Result<GrupoCofinsNt, String> {
        let codigo_situacao = parsercher::search_text_from_tag_children(&cofins, &Tag::new("CST"))
            .ok_or("Tag <CST> não encontrada na <COFINSNT>")?[0]
            .to_string();

        Ok(GrupoCofinsNt { codigo_situacao })
    }
}

/// Grupo COFINS Aliq - Aliq Operações
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoCofinsAliq {
    /// CST - Código de Situação Tributária do COFINS
    #[serde(rename = "CST")]
    pub codigo_situacao: String,
    /// Valor da base de cálculo do COFINS
    #[serde(rename = "vBC")]
    pub valor_base_calculo: f32,
    /// Alíquota do COFINS(%)
    #[serde(rename = "pCOFINS")]
    pub aliquota: f32,
    /// Valor do COFINS
    #[serde(rename = "vCOFINS")]
    pub valor: f32,
}

impl GrupoCofinsAliq {
    /// Parse do COFINSAliq
    pub(crate) fn parse(cofins: &Dom) -> Result<GrupoCofinsAliq, String> {
        let valor = parsercher::search_text_from_tag_children(&cofins, &Tag::new("vCOFINS"))
            .ok_or("Tag <vCOFINS> não encontrada na <COFINSAliq>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let aliquota = parsercher::search_text_from_tag_children(&cofins, &Tag::new("pCOFINS"))
            .ok_or("Tag <pCOFINS> não encontrada na <COFINSAliq>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_base_calculo =
            parsercher::search_text_from_tag_children(&cofins, &Tag::new("vBC"))
                .ok_or("Tag <vBC> não encontrada na <COFINSAliq>")?[0]
                .parse::<f32>()
                .map_err(|e| e.to_string())?;

        let codigo_situacao = parsercher::search_text_from_tag_children(&cofins, &Tag::new("CST"))
            .ok_or("Tag <CST> não encontrada na <COFINSAliq>")?[0]
            .to_string();

        Ok(GrupoCofinsAliq {
            aliquota,
            valor_base_calculo,
            codigo_situacao,
            valor,
        })
    }
}
