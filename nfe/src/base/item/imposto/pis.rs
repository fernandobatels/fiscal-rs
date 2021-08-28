/// Grupos de PIS
use serde::{Deserialize, Deserializer};

/// PIS
#[derive(Debug, PartialEq)]
pub enum GrupoPis {
    /// Outras Operações
    PisOutr(GrupoPisOutr),
    /// Não Tributado
    PisNt(GrupoPisNt),
    /// Tributado pela alíquota
    PisAliq(GrupoPisAliq),
}

impl<'de> Deserialize<'de> for GrupoPis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub enum TipoPis {
            #[serde(rename = "PISOutr")]
            PisOutr(GrupoPisOutr),
            #[serde(rename = "PISNT")]
            PisNt(GrupoPisNt),
            #[serde(rename = "PISAliq")]
            PisAliq(GrupoPisAliq),
        }

        #[derive(Deserialize)]
        struct GrupoPisContainer {
            #[serde(rename = "$value")]
            inner: TipoPis,
        }

        let gr = GrupoPisContainer::deserialize(deserializer)?;

        Ok(match gr.inner {
            TipoPis::PisOutr(g) => GrupoPis::PisOutr(g),
            TipoPis::PisNt(g) => GrupoPis::PisNt(g),
            TipoPis::PisAliq(g) => GrupoPis::PisAliq(g),
        })
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
