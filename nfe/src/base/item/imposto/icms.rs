//! Grupos de ICMS

use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;

/// ICMS
#[derive(Debug, PartialEq)]
pub enum GrupoIcms {
    /// Tributação ICMS pelo Simples Nacional, CSOSN=202 ou 203
    IcmsSn202(GrupoIcmsSn202),
    /// Tributação ICMS cobrado anteriormente por substituição tributária
    Icms60(GrupoIcms60),
}

impl<'de> Deserialize<'de> for GrupoIcms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub enum TipoIcms {
            #[serde(rename = "ICMSSN202")]
            IcmsSn202(GrupoIcmsSn202),
            #[serde(rename = "ICMS60")]
            Icms60(GrupoIcms60),
        }

        #[derive(Deserialize)]
        struct GrupoIcmsContainer {
            #[serde(rename = "$value")]
            inner: TipoIcms,
        }

        let gr = GrupoIcmsContainer::deserialize(deserializer)?;

        Ok(match gr.inner {
            TipoIcms::IcmsSn202(g) => GrupoIcms::IcmsSn202(g),
            TipoIcms::Icms60(g) => GrupoIcms::Icms60(g),
        })
    }
}

/// Grupo ICMS 60 - Tributação ICMS cobrado anteriormente por substituição tributária
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoIcms60 {
    /// Origem da mercadoria
    #[serde(rename = "orig")]
    pub origem: OrigemMercadoria,
    /// Valor da base de cálculo do ICMS ST retido
    #[serde(rename = "vBCSTRet")]
    pub valor_base_calculo: f32,
    /// Alíquota suportada pelo Consumidor Final
    #[serde(rename = "pST")]
    pub aliquota: f32,
    /// Valor do ICMS ST retido
    #[serde(rename = "vICMSSTRet")]
    pub valor: f32,
}

/// Tributação ICMS pelo Simples Nacional, CSOSN=202 ou 203
#[derive(Debug, PartialEq, Deserialize)]
pub struct GrupoIcmsSn202 {
    /// Origem da mercadoria
    #[serde(rename = "orig")]
    pub origem: OrigemMercadoria,
    /// Código de Situação da Operação – Simples Nacional
    #[serde(rename = "CSOSN")]
    pub codigo_situacao: String,
    /// Modalidade de determinação da BC do ICMS ST
    #[serde(rename = "modBCST")]
    pub base_calculo: ModalidadeBaseCalculoIcmsSt,
    /// Valor da base de cálculo
    #[serde(rename = "vBCST")]
    pub valor_base_calculo: f32,
    /// Alíquota do imposto do ICMS ST
    #[serde(rename = "pICMSST")]
    pub aliquota: f32,
    /// Valor do ICMS ST
    #[serde(rename = "vICMSST")]
    pub valor: f32,
}

/// Origem da mercadoria
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum OrigemMercadoria {
    /// Nacional, exceto as indicadas nos códigos 3, 4, 5 e 8
    Nacional = 0,
    /// Estrangeira - Importação direta, exceto a indicada no código 6
    Estrangeira = 1,
    /// Estrangeira - Adquirida no mercado interno, exceto a indicada no código 7
    EstrangeiraAdquiridaMercadoInterno = 2,
    /// Nacional, mercadoria ou bem com Conteúdo de Importação superior a 40% e inferior ou igual a 70%
    NacionalComImportacao40a70 = 3,
    /// Nacional, cuja produção tenha sido feita em conformidade com os processos produtivos básicos de que tratam as legislações citadas nos Ajustes
    NacionalProducaoEmConformidade = 4,
    /// Nacional, mercadoria ou bem com Conteúdo de Importação inferior ou igual a 40%
    NacionalComImportacaoInferior40 = 5,
    /// Estrangeira - Importação direta, sem similar nacional, constante em lista da CAMEX e gás natural
    EstrangeiraImportacaoDiretaSemSimilarNacional = 6,
    /// Estrangeira - Adquirida no mercado interno, sem similar nacional, constante lista CAMEX e gás natural.
    EstrangeiraAdquiridaMercadoInternoSemSimilarNacional = 7,
    /// Nacional, mercadoria ou bem com Conteúdo de Importação superior a 70%
    NacionalComImportacaoSuperior70 = 8,
}

/// Modalidade de determinação da BC do ICMS ST
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum ModalidadeBaseCalculoIcmsSt {
    /// Preço tabelado ou máximo sugerido
    PrecoTabelado = 0,
    /// Lista negativa (valor)
    ListaNegativa = 1,
    /// Lista positiva (valor)
    ListaPositiva = 2,
    /// Lista neutra (valor)
    ListaNeutra = 3,
    /// Margem valor agregado (%)
    MargemValorAgregado = 4,
    /// Pauta (valor)
    Pauta = 5,
}
