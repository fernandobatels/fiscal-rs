/// Grupos de COFINS
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

/// Grupo COFINS NT - COFINS não tributado
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoCofinsNt {
    /// CST - Código de Situação Tributária do COFINS
    #[serde(rename = "CST")]
    pub codigo_situacao: String,
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
