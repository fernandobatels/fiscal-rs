//! Totalização dos produtos e serviços

use super::Error;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

/// Totalização da nota fiscal
#[derive(Debug, PartialEq)]
pub struct Totalizacao {
    /// Base de cálculo do ICMS
    pub valor_base_calculo: f32,
    /// Valor total do ICMS
    pub valor_icms: f32,
    /// Valor total dos produtos e serviços
    pub valor_produtos: f32,
    /// Valor total do frete
    pub valor_frete: f32,
    /// Valor total do seguro
    pub valor_seguro: f32,
    /// Valor total do desconto
    pub valor_desconto: f32,
    /// Outras despesas acessórias
    pub valor_outros: f32,
    /// Valor total do PIS
    pub valor_pis: f32,
    /// Valor total do COFINS
    pub valor_cofins: f32,
    /// Valor total da nota
    pub valor_total: f32,
    /// Valor aproximado total de tributos federais, estaduais e municipais.
    pub valor_aproximado_tributos: f32,
}

impl FromStr for Totalizacao {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s).map_err(|e| e.into())
    }
}

impl<'de> Deserialize<'de> for Totalizacao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "total")]
        struct TotalContainer {
            #[serde(rename = "ICMSTot")]
            icms: IcmsTot,
        }

        #[derive(Deserialize)]
        struct IcmsTot {
            #[serde(rename = "vBC")]
            valor_base_calculo: f32,
            #[serde(rename = "vICMS")]
            valor_icms: f32,
            #[serde(rename = "vProd")]
            valor_produtos: f32,
            #[serde(rename = "vFrete")]
            valor_frete: f32,
            #[serde(rename = "vSeg")]
            valor_seguro: f32,
            #[serde(rename = "vDesc")]
            valor_desconto: f32,
            #[serde(rename = "vOutro")]
            valor_outros: f32,
            #[serde(rename = "vPIS")]
            valor_pis: f32,
            #[serde(rename = "vCOFINS")]
            valor_cofins: f32,
            #[serde(rename = "vNF")]
            valor_total: f32,
            #[serde(rename = "vTotTrib")]
            valor_aproximado_tributos: f32,
        }

        let helper = TotalContainer::deserialize(deserializer)?;
        Ok(Totalizacao {
            valor_base_calculo: helper.icms.valor_base_calculo,
            valor_icms: helper.icms.valor_icms,
            valor_produtos: helper.icms.valor_produtos,
            valor_frete: helper.icms.valor_frete,
            valor_seguro: helper.icms.valor_seguro,
            valor_desconto: helper.icms.valor_desconto,
            valor_outros: helper.icms.valor_outros,
            valor_pis: helper.icms.valor_pis,
            valor_cofins: helper.icms.valor_cofins,
            valor_total: helper.icms.valor_total,
            valor_aproximado_tributos: helper.icms.valor_aproximado_tributos,
        })
    }
}
