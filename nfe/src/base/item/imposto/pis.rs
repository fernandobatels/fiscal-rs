/// Grupos de PIS
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

/// Grupo PIS NT - PIS não tributado
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoPisNt {
    /// CST - Código de Situação Tributária do PIS
    #[serde(rename = "CST")]
    pub codigo_situacao: String,
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
