//! Endereço do emitente/destinatário da NF-e

use super::Error;
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::str::FromStr;

/// Representação de um endereço usado na NFe
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Endereco {
    #[serde(rename = "xLgr")]
    pub logradouro: String,
    #[serde(rename = "nro")]
    pub numero: String,
    #[serde(rename = "xCpl")]
    pub complemento: Option<String>,
    #[serde(rename = "xBairro")]
    pub bairro: String,
    #[serde(rename = "cMun")]
    pub codigo_municipio: u32,
    #[serde(rename = "xMun")]
    pub nome_municipio: String,
    #[serde(rename = "UF")]
    pub sigla_uf: String,
    #[serde(rename = "CEP")]
    pub cep: String,
    #[serde(rename = "cPais")]
    pub codigo_pais: u32,
    #[serde(rename = "xPais")]
    pub nome_pais: String,
    #[serde(rename = "fone")]
    pub telefone: Option<String>,
}

impl FromStr for Endereco {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl Serialize for Endereco {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(11))?;
        map.serialize_entry("xLgr", &self.logradouro)?;
        map.serialize_entry("nro", &self.numero)?;
        if let Some(cpl) = &self.complemento {
            map.serialize_entry("xCpl", cpl)?;
        }
        map.serialize_entry("xBairro", &self.bairro)?;
        map.serialize_entry("cMun", &self.codigo_municipio)?;
        map.serialize_entry("xMun", &self.nome_municipio)?;
        map.serialize_entry("UF", &self.sigla_uf)?;
        map.serialize_entry("CEP", &self.cep)?;
        map.serialize_entry("cPais", &self.codigo_pais)?;
        map.serialize_entry("xPais", &self.nome_pais)?;
        if let Some(fone) = &self.telefone {
            map.serialize_entry("fone", fone)?;
        }
        map.end()
    }
}

impl ToString for Endereco {
    fn to_string(&self) -> String {
        serde_xml_rs::to_string(self).expect("Falha ao serializar o endereço")
    }
}
