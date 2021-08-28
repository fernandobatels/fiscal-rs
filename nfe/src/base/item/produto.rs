//! Produtos

use super::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
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
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum EscalaRelevante {
    Sim = 1,
    Nao = 2,
}

impl FromStr for Produto {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Produto {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar o produto")
    }
}

impl<'de> Deserialize<'de> for Produto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO: voltar a tentar usar o serde flatten
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
            valor_compoe_total_nota: prod.valor_compoe_total_nota == 1,
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

impl Serialize for Produto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let prod = ProdContainer {
            codigo: self.codigo.clone(),
            gtin: match &self.gtin {
                Some(gt) => gt.clone(),
                None => "SEM GTIN".to_string(),
            },
            descricao: self.descricao.clone(),
            ncm: self.ncm.clone(),
            fabricante_cnpj: self.fabricante_cnpj.clone(),
            unidade: self.unidade.clone(),
            quantidade: self.quantidade.clone(),
            valor_unitario: self.valor_unitario.clone(),
            valor_bruto: self.valor_bruto,
            valor_frete: self.valor_frete,
            valor_seguro: self.valor_seguro,
            valor_desconto: self.valor_desconto,
            valor_outros: self.valor_outros,
            valor_compoe_total_nota: if self.valor_compoe_total_nota { 1 } else { 0 },
            t_cest: self.tributacao.cest.clone(),
            t_escala_relevante: self.tributacao.escala_relevante,
            t_codigo_beneficio_fiscal: self.tributacao.codigo_beneficio_fiscal.clone(),
            t_codigo_excecao_ipi: self.tributacao.codigo_excecao_ipi.clone(),
            t_cfop: self.tributacao.cfop.clone(),
            t_gtin: match &self.tributacao.gtin {
                Some(gt) => gt.clone(),
                None => "SEM GTIN".to_string(),
            },
            t_unidade: self.tributacao.unidade.clone(),
            t_quantidade: self.tributacao.quantidade,
            t_valor_unitario: self.tributacao.valor_unitario,
        };

        prod.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "prod")]
struct ProdContainer {
    #[serde(rename = "$unflatten=cProd")]
    pub codigo: String,
    #[serde(rename = "$unflatten=cEAN")]
    pub gtin: String,
    #[serde(rename = "$unflatten=xProd")]
    pub descricao: String,
    #[serde(rename = "$unflatten=NCM")]
    pub ncm: String,
    #[serde(rename = "$unflatten=CNPJFab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabricante_cnpj: Option<String>,
    #[serde(rename = "$unflatten=uCom")]
    pub unidade: String,
    #[serde(rename = "$unflatten=qCom")]
    pub quantidade: f32,
    #[serde(rename = "$unflatten=vUnCom")]
    pub valor_unitario: f32,
    #[serde(rename = "$unflatten=vProd")]
    pub valor_bruto: f32,
    #[serde(rename = "$unflatten=vFrete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_frete: Option<f32>,
    #[serde(rename = "$unflatten=vDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_seguro: Option<f32>,
    #[serde(rename = "$unflatten=vSeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_desconto: Option<f32>,
    #[serde(rename = "$unflatten=vOutro")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valor_outros: Option<f32>,
    #[serde(rename = "$unflatten=indTot")]
    pub valor_compoe_total_nota: u8,

    #[serde(rename = "$unflatten=CEST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_cest: Option<String>,
    #[serde(rename = "$unflatten=indEscala")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_escala_relevante: Option<EscalaRelevante>,
    #[serde(rename = "$unflatten=cBenef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_codigo_beneficio_fiscal: Option<String>,
    #[serde(rename = "$unflatten=EXTIPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_codigo_excecao_ipi: Option<String>,
    #[serde(rename = "$unflatten=CFOP")]
    pub t_cfop: String,
    #[serde(rename = "$unflatten=cEANTrib")]
    pub t_gtin: String,
    #[serde(rename = "$unflatten=uTrib")]
    pub t_unidade: String,
    #[serde(rename = "$unflatten=qTrib")]
    pub t_quantidade: f32,
    #[serde(rename = "$unflatten=vUnTrib")]
    pub t_valor_unitario: f32,
}
