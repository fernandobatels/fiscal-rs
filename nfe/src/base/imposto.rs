//! Impostos dos itens

use parsercher::dom::*;
use std::str::FromStr;

/// Detalhamentos impostos sobre o item
pub struct Imposto {
    /// Valor aproximado total de tributos federais, estaduais e municipais
    pub valor_aproximado: Option<f32>,
    /// Informações do ICMS da Operação própria e ST
    pub icms: Option<GrupoIcms>,
    /// Informações do PIS
    pub pis: Option<GrupoPis>,
}

impl Imposto {
    /// Parse dos impostos do item
    pub(crate) fn parse(xml: &Dom) -> Result<Imposto, String> {
        let mut t_imposto = Dom::new(DomType::Tag);
        t_imposto.set_tag(Tag::new("imposto"));

        let imposto =
            parsercher::search_dom(&xml, &t_imposto).ok_or("Tag <imposto> não encontrada")?;

        let valor_aproximado = {
            if let Some(vl) =
                parsercher::search_text_from_tag_children(&imposto, &Tag::new("vTotTrib"))
            {
                Some(vl[0].parse::<f32>().map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let icms = GrupoIcms::parse(&imposto)?;

        let pis = GrupoPis::parse(&imposto)?;

        Ok(Imposto {
            valor_aproximado,
            icms,
            pis,
        })
    }
}

/// ICMS
#[derive(Debug, PartialEq)]
pub enum GrupoIcms {
    /// Tributação ICMS pelo Simples Nacional, CSOSN=202 ou 203
    IcmsSn202(GrupoIcmsSn202),
    /// Tributação ICMS cobrado anteriormente por substituição tributária
    Icms60(GrupoIcms60),
}

impl GrupoIcms {
    /// Parse dos tipos de ICMS do item
    pub(crate) fn parse(imposto: &Dom) -> Result<Option<GrupoIcms>, String> {
        let mut t_icms = Dom::new(DomType::Tag);

        t_icms.set_tag(Tag::new("ICMSSN202"));
        if let Some(icms) = parsercher::search_dom(&imposto, &t_icms) {
            return Ok(Some(GrupoIcms::IcmsSn202(GrupoIcmsSn202::parse(&icms)?)));
        }

        t_icms.set_tag(Tag::new("ICMS60"));
        if let Some(icms) = parsercher::search_dom(&imposto, &t_icms) {
            return Ok(Some(GrupoIcms::Icms60(GrupoIcms60::parse(&icms)?)));
        }

        Ok(None)
    }
}

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

impl GrupoPis {
    /// Parse dos tipos de PIS do item
    pub(crate) fn parse(imposto: &Dom) -> Result<Option<GrupoPis>, String> {
        let mut t_pis = Dom::new(DomType::Tag);

        t_pis.set_tag(Tag::new("PISOutr"));
        if let Some(pis) = parsercher::search_dom(&imposto, &t_pis) {
            return Ok(Some(GrupoPis::PisOutr(GrupoPisOutr::parse(&pis)?)));
        }

        t_pis.set_tag(Tag::new("PISNT"));
        if let Some(pis) = parsercher::search_dom(&imposto, &t_pis) {
            return Ok(Some(GrupoPis::PisNt(GrupoPisNt::parse(&pis)?)));
        }

        t_pis.set_tag(Tag::new("PISAliq"));
        if let Some(pis) = parsercher::search_dom(&imposto, &t_pis) {
            return Ok(Some(GrupoPis::PisAliq(GrupoPisAliq::parse(&pis)?)));
        }

        Ok(None)
    }
}

/// Grupo ICMS 60 - Tributação ICMS cobrado anteriormente por substituição tributária
#[derive(Debug, PartialEq)]
pub struct GrupoIcms60 {
    /// Origem da mercadoria
    pub origem: OrigemMercadoria,
    /// Valor da base de cálculo do ICMS ST retido
    pub valor_base_calculo: f32,
    /// Alíquota suportada pelo Consumidor Final
    pub aliquota: f32,
    /// Valor do ICMS ST retido
    pub valor: f32,
}

impl GrupoIcms60 {
    /// Parse do ICMS60
    pub(crate) fn parse(icms: &Dom) -> Result<GrupoIcms60, String> {
        let origem = parsercher::search_text_from_tag_children(&icms, &Tag::new("orig"))
            .ok_or("Tag <orig> não encontrada na <ICMS60>")?[0]
            .parse::<OrigemMercadoria>()?;

        let valor = parsercher::search_text_from_tag_children(&icms, &Tag::new("vICMSSTRet"))
            .ok_or("Tag <vICMSSTRet> não encontrada na <ICMS60>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let aliquota = parsercher::search_text_from_tag_children(&icms, &Tag::new("pST"))
            .ok_or("Tag <pST> não encontrada na <ICMS60>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_base_calculo =
            parsercher::search_text_from_tag_children(&icms, &Tag::new("vBCSTRet"))
                .ok_or("Tag <vBCSTRet> não encontrada na <ICMS60>")?[0]
                .parse::<f32>()
                .map_err(|e| e.to_string())?;

        Ok(GrupoIcms60 {
            valor,
            aliquota,
            valor_base_calculo,
            origem,
        })
    }
}

/// Tributação ICMS pelo Simples Nacional, CSOSN=202 ou 203
#[derive(Debug, PartialEq)]
pub struct GrupoIcmsSn202 {
    /// Origem da mercadoria
    pub origem: OrigemMercadoria,
    /// Código de Situação da Operação – Simples Nacional
    pub codigo_situacao: String,
    /// Modalidade de determinação da BC do ICMS ST
    pub base_calculo: ModalidadeBaseCalculoIcmsSt,
    /// Valor da base de cálculo
    pub valor_base_calculo: f32,
    /// Alíquota do imposto do ICMS ST
    pub aliquota: f32,
    /// Valor do ICMS ST
    pub valor: f32,
}

impl GrupoIcmsSn202 {
    /// Parse do ICMSSN202
    pub(crate) fn parse(icms: &Dom) -> Result<GrupoIcmsSn202, String> {
        let origem = parsercher::search_text_from_tag_children(&icms, &Tag::new("orig"))
            .ok_or("Tag <orig> não encontrada na <ICMSSN202>")?[0]
            .parse::<OrigemMercadoria>()?;

        let valor = parsercher::search_text_from_tag_children(&icms, &Tag::new("vICMSST"))
            .ok_or("Tag <vICMSST> não encontrada na <ICMSSN202>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let aliquota = parsercher::search_text_from_tag_children(&icms, &Tag::new("pICMSST"))
            .ok_or("Tag <pICMSST> não encontrada na <ICMSSN202>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_base_calculo =
            parsercher::search_text_from_tag_children(&icms, &Tag::new("vBCST"))
                .ok_or("Tag <vBCST> não encontrada na <ICMSSN202>")?[0]
                .parse::<f32>()
                .map_err(|e| e.to_string())?;

        let codigo_situacao = parsercher::search_text_from_tag_children(&icms, &Tag::new("CSOSN"))
            .ok_or("Tag <CSOSN> não encontrada na <ICMSSN202>")?[0]
            .to_string();

        let base_calculo = parsercher::search_text_from_tag_children(&icms, &Tag::new("modBCST"))
            .ok_or("Tag <modBCST> não encontrada na <ICMSSN202>")?[0]
            .parse::<ModalidadeBaseCalculoIcmsSt>()?;

        Ok(GrupoIcmsSn202 {
            valor,
            aliquota,
            valor_base_calculo,
            codigo_situacao,
            base_calculo,
            origem,
        })
    }
}

/// Grupo PIS Outr - Outras Operações
#[derive(Debug, PartialEq)]
pub struct GrupoPisOutr {
    /// CST - Código de Situação Tributária do PIS
    pub codigo_situacao: String,
    /// Valor da base de cálculo do PIS
    pub valor_base_calculo: f32,
    /// Alíquota do PIS(%)
    pub aliquota: f32,
}

impl GrupoPisOutr {
    /// Parse do PISOutr
    pub(crate) fn parse(pis: &Dom) -> Result<GrupoPisOutr, String> {
        let aliquota = parsercher::search_text_from_tag_children(&pis, &Tag::new("pPIS"))
            .ok_or("Tag <pPIS> não encontrada na <PISOutr>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_base_calculo = parsercher::search_text_from_tag_children(&pis, &Tag::new("vBC"))
            .ok_or("Tag <vBC> não encontrada na <PISOutr>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let codigo_situacao = parsercher::search_text_from_tag_children(&pis, &Tag::new("CST"))
            .ok_or("Tag <CST> não encontrada na <PISOutr>")?[0]
            .to_string();

        Ok(GrupoPisOutr {
            aliquota,
            valor_base_calculo,
            codigo_situacao,
        })
    }
}

/// Grupo PIS NT - PIS não tributado
#[derive(Debug, PartialEq)]
pub struct GrupoPisNt {
    /// CST - Código de Situação Tributária do PIS
    pub codigo_situacao: String,
}

impl GrupoPisNt {
    /// Parse do PISNT
    pub(crate) fn parse(pis: &Dom) -> Result<GrupoPisNt, String> {
        let codigo_situacao = parsercher::search_text_from_tag_children(&pis, &Tag::new("CST"))
            .ok_or("Tag <CST> não encontrada na <PISNT>")?[0]
            .to_string();

        Ok(GrupoPisNt { codigo_situacao })
    }
}

/// Grupo PIS Aliq - Aliq Operações
#[derive(Debug, PartialEq)]
pub struct GrupoPisAliq {
    /// CST - Código de Situação Tributária do PIS
    pub codigo_situacao: String,
    /// Valor da base de cálculo do PIS
    pub valor_base_calculo: f32,
    /// Alíquota do PIS(%)
    pub aliquota: f32,
    /// Valor do PIS
    pub valor: f32,
}

impl GrupoPisAliq {
    /// Parse do PISAliq
    pub(crate) fn parse(pis: &Dom) -> Result<GrupoPisAliq, String> {
        let valor = parsercher::search_text_from_tag_children(&pis, &Tag::new("vPIS"))
            .ok_or("Tag <vPIS> não encontrada na <PISAliq>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let aliquota = parsercher::search_text_from_tag_children(&pis, &Tag::new("pPIS"))
            .ok_or("Tag <pPIS> não encontrada na <PISAliq>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_base_calculo = parsercher::search_text_from_tag_children(&pis, &Tag::new("vBC"))
            .ok_or("Tag <vBC> não encontrada na <PISAliq>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let codigo_situacao = parsercher::search_text_from_tag_children(&pis, &Tag::new("CST"))
            .ok_or("Tag <CST> não encontrada na <PISAliq>")?[0]
            .to_string();

        Ok(GrupoPisAliq {
            aliquota,
            valor_base_calculo,
            codigo_situacao,
            valor,
        })
    }
}

/// Origem da mercadoria
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

impl FromStr for OrigemMercadoria {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "8" => OrigemMercadoria::NacionalComImportacaoSuperior70,
            "7" => OrigemMercadoria::EstrangeiraAdquiridaMercadoInterno,
            "6" => OrigemMercadoria::EstrangeiraImportacaoDiretaSemSimilarNacional,
            "5" => OrigemMercadoria::NacionalComImportacaoInferior40,
            "4" => OrigemMercadoria::NacionalProducaoEmConformidade,
            "3" => OrigemMercadoria::NacionalComImportacao40a70,
            "2" => OrigemMercadoria::EstrangeiraAdquiridaMercadoInterno,
            "1" => OrigemMercadoria::Estrangeira,
            _ => OrigemMercadoria::Nacional, // 0
        })
    }
}

/// Modalidade de determinação da BC do ICMS ST
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

impl FromStr for ModalidadeBaseCalculoIcmsSt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "5" => ModalidadeBaseCalculoIcmsSt::Pauta,
            "4" => ModalidadeBaseCalculoIcmsSt::MargemValorAgregado,
            "3" => ModalidadeBaseCalculoIcmsSt::ListaNeutra,
            "2" => ModalidadeBaseCalculoIcmsSt::ListaPositiva,
            "1" => ModalidadeBaseCalculoIcmsSt::ListaNegativa,
            _ => ModalidadeBaseCalculoIcmsSt::PrecoTabelado, // 0
        })
    }
}
