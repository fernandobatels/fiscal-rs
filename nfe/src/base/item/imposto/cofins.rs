/// Grupos de COFINS
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

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
        let grc = GrupoCofinsContainer::deserialize(deserializer)?;

        if let Some(gr) = grc.cofins_outr {
            return Ok(GrupoCofins::CofinsOutr(gr));
        }

        if let Some(gr) = grc.cofins_nt {
            return Ok(GrupoCofins::CofinsNt(gr));
        }

        if let Some(gr) = grc.cofins_aliq {
            return Ok(GrupoCofins::CofinsAliq(gr));
        }

        Err(Error::custom("Tipo de COFINS não suportado".to_string()))
    }
}

impl Serialize for GrupoCofins {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let grc = match self {
            GrupoCofins::CofinsOutr(g) => GrupoCofinsContainer {
                cofins_outr: Some(g.clone()),
                cofins_nt: None,
                cofins_aliq: None,
            },
            GrupoCofins::CofinsNt(g) => GrupoCofinsContainer {
                cofins_outr: None,
                cofins_nt: Some(g.clone()),
                cofins_aliq: None,
            },
            GrupoCofins::CofinsAliq(g) => GrupoCofinsContainer {
                cofins_outr: None,
                cofins_nt: None,
                cofins_aliq: Some(g.clone()),
            },
        };

        grc.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
struct GrupoCofinsContainer {
    #[serde(rename = "COFINSOutr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cofins_outr: Option<GrupoCofinsOutr>,
    #[serde(rename = "COFINSNT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cofins_nt: Option<GrupoCofinsNt>,
    #[serde(rename = "COFINSAliq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cofins_aliq: Option<GrupoCofinsAliq>,
}

/// Grupo COFINS Outr - Outras Operações
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GrupoCofinsOutr {
    /// CST - Código de Situação Tributária do COFINS
    #[serde(rename = "$unflatten=CST")]
    pub codigo_situacao: String,
    /// Valor da base de cálculo do COFINS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_base_calculo: f32,
    /// Alíquota do COFINS(%)
    #[serde(rename = "$unflatten=pCOFINS")]
    pub aliquota: f32,
}

impl Clone for GrupoCofinsOutr {
    fn clone(&self) -> Self {
        Self {
            codigo_situacao: self.codigo_situacao.clone(),
            valor_base_calculo: self.valor_base_calculo,
            aliquota: self.aliquota,
        }
    }
}

/// Grupo COFINS NT - COFINS não tributado
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GrupoCofinsNt {
    /// CST - Código de Situação Tributária do COFINS
    #[serde(rename = "$unflatten=CST")]
    pub codigo_situacao: String,
}

impl Clone for GrupoCofinsNt {
    fn clone(&self) -> Self {
        Self {
            codigo_situacao: self.codigo_situacao.clone(),
        }
    }
}

/// Grupo COFINS Aliq - Aliq Operações
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GrupoCofinsAliq {
    /// CST - Código de Situação Tributária do COFINS
    #[serde(rename = "$unflatten=CST")]
    pub codigo_situacao: String,
    /// Valor da base de cálculo do COFINS
    #[serde(rename = "$unflatten=vBC")]
    pub valor_base_calculo: f32,
    /// Alíquota do COFINS(%)
    #[serde(rename = "$unflatten=pCOFINS")]
    pub aliquota: f32,
    /// Valor do COFINS
    #[serde(rename = "$unflatten=vCOFINS")]
    pub valor: f32,
}

impl Clone for GrupoCofinsAliq {
    fn clone(&self) -> Self {
        Self {
            codigo_situacao: self.codigo_situacao.clone(),
            valor_base_calculo: self.valor_base_calculo,
            aliquota: self.aliquota,
            valor: self.valor,
        }
    }
}
