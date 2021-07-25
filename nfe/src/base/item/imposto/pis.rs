/// Grupos de PIS
use parsercher::dom::*;
use serde::Deserialize;

/// PIS
#[derive(Debug, PartialEq, Deserialize)]
pub enum GrupoPis {
    /// Outras Operações
    #[serde(rename = "PISOutr")]
    PisOutr(GrupoPisOutr),
    /// Não Tributado
    #[serde(rename = "PISNT")]
    PisNt(GrupoPisNt),
    /// Tributado pela alíquota
    #[serde(rename = "PISAliq")]
    PisAliq(GrupoPisAliq),
}

impl GrupoPis {
    /// Parse dos tipos de PIS do item
    pub(crate) fn parse(imposto: &Dom) -> Result<Option<GrupoPis>, String> {
        let mut t_pis = Dom::new(DomType::Tag);

        t_pis.set_tag(Tag::new("PISOutr"));
        if let Some(pis) = parsercher::search_dom(&imposto, &t_pis) {
            return Ok(Some(GrupoPis::PisOutr(GrupoPisOutr::parse(&pis)?)));
        }

        t_pis.set_tag(Tag::new("PISNT"));
        if let Some(pis) = parsercher::search_dom(&imposto, &t_pis) {
            return Ok(Some(GrupoPis::PisNt(GrupoPisNt::parse(&pis)?)));
        }

        t_pis.set_tag(Tag::new("PISAliq"));
        if let Some(pis) = parsercher::search_dom(&imposto, &t_pis) {
            return Ok(Some(GrupoPis::PisAliq(GrupoPisAliq::parse(&pis)?)));
        }

        Ok(None)
    }
}

/// Grupo PIS Outr - Outras Operações
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoPisOutr {
    /// CST - Código de Situação Tributária do PIS
    #[serde(rename = "CST")]
    pub codigo_situacao: String,
    /// Valor da base de cálculo do PIS
    #[serde(rename = "vBC")]
    pub valor_base_calculo: f32,
    /// Alíquota do PIS(%)
    #[serde(rename = "pPIS")]
    pub aliquota: f32,
}

impl GrupoPisOutr {
    /// Parse do PISOutr
    pub(crate) fn parse(pis: &Dom) -> Result<GrupoPisOutr, String> {
        let aliquota = parsercher::search_text_from_tag_children(&pis, &Tag::new("pPIS"))
            .ok_or("Tag <pPIS> não encontrada na <PISOutr>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_base_calculo = parsercher::search_text_from_tag_children(&pis, &Tag::new("vBC"))
            .ok_or("Tag <vBC> não encontrada na <PISOutr>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let codigo_situacao = parsercher::search_text_from_tag_children(&pis, &Tag::new("CST"))
            .ok_or("Tag <CST> não encontrada na <PISOutr>")?[0]
            .to_string();

        Ok(GrupoPisOutr {
            aliquota,
            valor_base_calculo,
            codigo_situacao,
        })
    }
}

/// Grupo PIS NT - PIS não tributado
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoPisNt {
    /// CST - Código de Situação Tributária do PIS
    #[serde(rename = "CST")]
    pub codigo_situacao: String,
}

impl GrupoPisNt {
    /// Parse do PISNT
    pub(crate) fn parse(pis: &Dom) -> Result<GrupoPisNt, String> {
        let codigo_situacao = parsercher::search_text_from_tag_children(&pis, &Tag::new("CST"))
            .ok_or("Tag <CST> não encontrada na <PISNT>")?[0]
            .to_string();

        Ok(GrupoPisNt { codigo_situacao })
    }
}

/// Grupo PIS Aliq - Aliq Operações
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoPisAliq {
    /// CST - Código de Situação Tributária do PIS
    #[serde(rename = "CST")]
    pub codigo_situacao: String,
    /// Valor da base de cálculo do PIS
    #[serde(rename = "vBC")]
    pub valor_base_calculo: f32,
    /// Alíquota do PIS(%)
    #[serde(rename = "pPIS")]
    pub aliquota: f32,
    /// Valor do PIS
    #[serde(rename = "vPIS")]
    pub valor: f32,
}

impl GrupoPisAliq {
    /// Parse do PISAliq
    pub(crate) fn parse(pis: &Dom) -> Result<GrupoPisAliq, String> {
        let valor = parsercher::search_text_from_tag_children(&pis, &Tag::new("vPIS"))
            .ok_or("Tag <vPIS> não encontrada na <PISAliq>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let aliquota = parsercher::search_text_from_tag_children(&pis, &Tag::new("pPIS"))
            .ok_or("Tag <pPIS> não encontrada na <PISAliq>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_base_calculo = parsercher::search_text_from_tag_children(&pis, &Tag::new("vBC"))
            .ok_or("Tag <vBC> não encontrada na <PISAliq>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let codigo_situacao = parsercher::search_text_from_tag_children(&pis, &Tag::new("CST"))
            .ok_or("Tag <CST> não encontrada na <PISAliq>")?[0]
            .to_string();

        Ok(GrupoPisAliq {
            aliquota,
            valor_base_calculo,
            codigo_situacao,
            valor,
        })
    }
}
