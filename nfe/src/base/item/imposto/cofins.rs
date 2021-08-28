/// Grupos de COFINS
use serde::{Deserialize, Deserializer};

/// COFINS
#[derive(Debug, PartialEq)]
pub enum GrupoCofins {
    /// Outras Operações
    CofinsOutr(GrupoCofinsOutr),
    /// Não Tributado
    CofinsNt(GrupoCofinsNt),
    /// Tributado pela alíquota
    CofinsAliq(GrupoCofinsAliq),
}

impl<'de> Deserialize<'de> for GrupoCofins {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

        #[derive(Deserialize)]
        pub enum TipoCofins {
            #[serde(rename = "COFINSOutr")]
            CofinsOutr(GrupoCofinsOutr),
            #[serde(rename = "COFINSNT")]
            CofinsNt(GrupoCofinsNt),
            #[serde(rename = "COFINSAliq")]
            CofinsAliq(GrupoCofinsAliq),
        }

        #[derive(Deserialize)]
        struct GrupoCofinsContainer{
            #[serde(rename = "$value")]
            inner: TipoCofins
        }

        let gr = GrupoCofinsContainer::deserialize(deserializer)?;

        Ok(match gr.inner {
            TipoCofins::CofinsOutr(g) => GrupoCofins::CofinsOutr(g),
            TipoCofins::CofinsNt(g) => GrupoCofins::CofinsNt(g),
            TipoCofins::CofinsAliq(g) => GrupoCofins::CofinsAliq(g)
        })
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
