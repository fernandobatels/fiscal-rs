/// Grupos de PIS
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// PIS
#[derive(Debug, PartialEq, Clone)]
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
        let grc = GrupoPisContainer::deserialize(deserializer)?;

        if let Some(gr) = grc.pis_outr {
            return Ok(GrupoPis::PisOutr(gr));
        }

        if let Some(gr) = grc.pis_nt {
            return Ok(GrupoPis::PisNt(gr));
        }

        if let Some(gr) = grc.pis_aliq {
            return Ok(GrupoPis::PisAliq(gr));
        }

        Err(Error::custom("Tipo de PIS não suportado".to_string()))
    }
}

impl Serialize for GrupoPis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let grc = match self {
            GrupoPis::PisOutr(g) => GrupoPisContainer {
                pis_outr: Some(g.clone()),
                pis_nt: None,
                pis_aliq: None,
            },
            GrupoPis::PisNt(g) => GrupoPisContainer {
                pis_outr: None,
                pis_nt: Some(g.clone()),
                pis_aliq: None,
            },
            GrupoPis::PisAliq(g) => GrupoPisContainer {
                pis_outr: None,
                pis_nt: None,
                pis_aliq: Some(g.clone()),
            },
        };

        grc.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
struct GrupoPisContainer {
    #[serde(rename = "PISOutr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pis_outr: Option<GrupoPisOutr>,
    #[serde(rename = "PISNT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pis_nt: Option<GrupoPisNt>,
    #[serde(rename = "PISAliq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pis_aliq: Option<GrupoPisAliq>,
}

/// Grupo PIS Outr - Outras Operações
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct GrupoPisOutr {
    /// CST - Código de Situação Tributária do PIS
    #[serde(rename = "$unflatten=CST")]
    pub codigo_situacao: String,
    /// Valor da base de cálculo do PIS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_base_calculo: f32,
    /// Alíquota do PIS(%)
    #[serde(rename = "$unflatten=pPIS")]
    pub aliquota: f32,
}

/// Grupo PIS NT - PIS não tributado
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct GrupoPisNt {
    /// CST - Código de Situação Tributária do PIS
    #[serde(rename = "$unflatten=CST")]
    pub codigo_situacao: String,
}

/// Grupo PIS Aliq - Aliq Operações
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct GrupoPisAliq {
    /// CST - Código de Situação Tributária do PIS
    #[serde(rename = "$unflatten=CST")]
    pub codigo_situacao: String,
    /// Valor da base de cálculo do PIS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_base_calculo: f32,
    /// Alíquota do PIS(%)
    #[serde(rename = "$unflatten=pPIS")]
    pub aliquota: f32,
    /// Valor do PIS
    #[serde(rename = "$unflatten=vPIS")]
    pub valor: f32,
}
