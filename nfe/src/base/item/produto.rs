//! Produtos

use super::Error;
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use std::str::FromStr;

/// Detalhamento do produto do item
#[derive(Debug, PartialEq)]
pub struct Produto {
    /// Código do produto
    pub codigo: String,
    /// GTIN (Global Trade Item Number) do produto, antigo código EAN ou código de barras
    pub gtin: Option<String>,
    /// Descrição do produto
    pub descricao: String,
    /// NCM - Nomenclatura Comum do Mercosul
    pub ncm: String,
    /// CNPJ do Fabricante da Mercadoria
    pub fabricante_cnpj: Option<String>,
    /// Dados sobre a tributação do produto
    pub tributacao: ProdutoTributacao,
    /// Unidade de medida da comercialização
    pub unidade: String,
    /// Quantidade da comercialização do produto
    pub quantidade: f32,
    /// Valor unitário do produto
    pub valor_unitario: f32,
    /// Valor total bruto do produto. ICMS incluso
    pub valor_bruto: f32,
    /// Valor total do frete do produto
    pub valor_frete: Option<f32>,
    /// Valor total do seguro do produto
    pub valor_seguro: Option<f32>,
    /// Valor total desconto
    pub valor_desconto: Option<f32>,
    /// Outras despesas acessórias
    pub valor_outros: Option<f32>,
    /// Indica se valor bruto entra no valor total da NF-e
    pub valor_compoe_total_nota: bool,
}

/// Dados sobre a tributação do produto
#[derive(Debug, PartialEq)]
pub struct ProdutoTributacao {
    /// CEST - Código Especificador da Substituição Tributária
    pub cest: Option<String>,
    /// Indicador de Produção em escala relevante
    pub escala_relevante: Option<EscalaRelevante>,
    /// Código de Benefício Fiscal na UF aplicado ao item
    pub codigo_beneficio_fiscal: Option<String>,
    /// Código Exceção da Tabela de IPI
    pub codigo_excecao_ipi: Option<String>,
    /// Código Fiscal de Operações e Prestações
    pub cfop: String,
    /// GTIN (Global Trade Item Number) da unidade tributável do produto
    pub gtin: Option<String>,
    /// Unidade tributável
    pub unidade: String,
    /// Quantidade tributável
    pub quantidade: f32,
    /// Valor unitário de tributação
    pub valor_unitario: f32,
}

/// Indicador de Produção em escala relevante
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum EscalaRelevante {
    Sim = 1,
    Nao = 2,
}

impl FromStr for Produto {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s).map_err(|e| e.into())
    }
}

impl<'de> Deserialize<'de> for Produto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO: voltar a tentar usar o serde flatten

        #[derive(Deserialize)]
        struct ProdContainer {
            #[serde(rename = "cProd")]
            pub codigo: String,
            #[serde(rename = "cEAN")]
            pub gtin: String,
            #[serde(rename = "xProd")]
            pub descricao: String,
            #[serde(rename = "NCM")]
            pub ncm: String,
            #[serde(rename = "CNPJFab")]
            pub fabricante_cnpj: Option<String>,
            #[serde(rename = "uCom")]
            pub unidade: String,
            #[serde(rename = "qCom")]
            pub quantidade: f32,
            #[serde(rename = "vUnCom")]
            pub valor_unitario: f32,
            #[serde(rename = "vProd")]
            pub valor_bruto: f32,
            #[serde(rename = "vFrete")]
            pub valor_frete: Option<f32>,
            #[serde(rename = "vDesc")]
            pub valor_seguro: Option<f32>,
            #[serde(rename = "vSeg")]
            pub valor_desconto: Option<f32>,
            #[serde(rename = "vOutro")]
            pub valor_outros: Option<f32>,
            #[serde(rename = "indTot")]
            pub valor_compoe_total_nota: bool,

            #[serde(rename = "CEST")]
            pub t_cest: Option<String>,
            #[serde(rename = "indEscala")]
            pub t_escala_relevante: Option<EscalaRelevante>,
            #[serde(rename = "cBenef")]
            pub t_codigo_beneficio_fiscal: Option<String>,
            #[serde(rename = "EXTIPI")]
            pub t_codigo_excecao_ipi: Option<String>,
            #[serde(rename = "CFOP")]
            pub t_cfop: String,
            #[serde(rename = "cEANTrib")]
            pub t_gtin: String,
            #[serde(rename = "uTrib")]
            pub t_unidade: String,
            #[serde(rename = "qTrib")]
            pub t_quantidade: f32,
            #[serde(rename = "vUnTrib")]
            pub t_valor_unitario: f32,
        }

        let prod = ProdContainer::deserialize(deserializer)?;

        Ok(Self {
            codigo: prod.codigo,
            gtin: match prod.gtin.to_lowercase().trim() {
                "sem gtin" => None,
                "" => None,
                _ => Some(prod.gtin),
            },
            descricao: prod.descricao,
            ncm: prod.ncm,
            fabricante_cnpj: prod.fabricante_cnpj,
            unidade: prod.unidade,
            quantidade: prod.quantidade,
            valor_unitario: prod.valor_unitario,
            valor_bruto: prod.valor_bruto,
            valor_frete: prod.valor_frete,
            valor_seguro: prod.valor_seguro,
            valor_desconto: prod.valor_desconto,
            valor_outros: prod.valor_outros,
            valor_compoe_total_nota: prod.valor_compoe_total_nota,
            tributacao: ProdutoTributacao {
                cest: prod.t_cest,
                escala_relevante: prod.t_escala_relevante,
                codigo_beneficio_fiscal: prod.t_codigo_beneficio_fiscal,
                codigo_excecao_ipi: prod.t_codigo_excecao_ipi,
                cfop: prod.t_cfop,
                gtin: match prod.t_gtin.to_lowercase().trim() {
                    "sem gtin" => None,
                    "" => None,
                    _ => Some(prod.t_gtin),
                },
                unidade: prod.t_unidade,
                quantidade: prod.t_quantidade,
                valor_unitario: prod.t_valor_unitario,
            },
        })
    }
}
