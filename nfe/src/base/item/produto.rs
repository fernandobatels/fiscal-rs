//! Produtos

use parsercher::dom::*;
use std::str::FromStr;

/// Detalhamento do produto do item
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

impl Produto {
    /// Parse do produto do item
    pub(crate) fn parse(xml: &Dom) -> Result<Produto, String> {
        let mut t_prod = Dom::new(DomType::Tag);
        t_prod.set_tag(Tag::new("prod"));

        let prod = parsercher::search_dom(&xml, &t_prod).ok_or("Tag <prod> não encontrada")?;

        let codigo = parsercher::search_text_from_tag_children(&prod, &Tag::new("cProd"))
            .ok_or("Tag <cProd> não encontrada na <prod>")?[0]
            .to_string();

        let gtin = {
            if let Some(ean) = parsercher::search_text_from_tag_children(&prod, &Tag::new("cEAN")) {
                let gtin = ean[0].to_string();

                match gtin.to_lowercase().trim() {
                    "sem gtin" => None,
                    _ => Some(gtin),
                }
            } else {
                None
            }
        };

        let descricao = parsercher::search_text_from_tag_children(&prod, &Tag::new("xProd"))
            .ok_or("Tag <xProd> não encontrada na <prod>")?[0]
            .to_string();

        let ncm = parsercher::search_text_from_tag_children(&prod, &Tag::new("NCM"))
            .ok_or("Tag <NCM> não encontrada na <prod>")?[0]
            .to_string();

        let fabricante_cnpj = {
            if let Some(fa) = parsercher::search_text_from_tag_children(&prod, &Tag::new("CNPJFab"))
            {
                Some(fa[0].to_string())
            } else {
                None
            }
        };

        let unidade = parsercher::search_text_from_tag_children(&prod, &Tag::new("uCom"))
            .ok_or("Tag <uCom> não encontrada na <prod>")?[0]
            .to_string();

        let quantidade = parsercher::search_text_from_tag_children(&prod, &Tag::new("qCom"))
            .ok_or("Tag <qCom> não encontrada na <prod>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_unitario = parsercher::search_text_from_tag_children(&prod, &Tag::new("vUnCom"))
            .ok_or("Tag <vUnCom> não encontrada na <prod>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let tributacao = ProdutoTributacao::parse(&prod)?;

        let valor_bruto = parsercher::search_text_from_tag_children(&prod, &Tag::new("vProd"))
            .ok_or("Tag <vProd> não encontrada na <prod>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_frete = {
            if let Some(vl) = parsercher::search_text_from_tag_children(&prod, &Tag::new("vFrete"))
            {
                Some(vl[0].parse::<f32>().map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let valor_desconto = {
            if let Some(vl) = parsercher::search_text_from_tag_children(&prod, &Tag::new("vDesc")) {
                Some(vl[0].parse::<f32>().map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let valor_seguro = {
            if let Some(vl) = parsercher::search_text_from_tag_children(&prod, &Tag::new("vSeg")) {
                Some(vl[0].parse::<f32>().map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let valor_outros = {
            if let Some(vl) = parsercher::search_text_from_tag_children(&prod, &Tag::new("vOutro"))
            {
                Some(vl[0].parse::<f32>().map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let valor_compoe_total_nota = {
            let ind = parsercher::search_text_from_tag_children(&prod, &Tag::new("indTot"))
                .ok_or("Tag <indTot> não encontrada na <prod>")?[0]
                .to_string();

            ind == "1"
        };

        Ok(Produto {
            codigo,
            gtin,
            descricao,
            ncm,
            fabricante_cnpj,
            unidade,
            quantidade,
            valor_unitario,
            tributacao,
            valor_bruto,
            valor_frete,
            valor_desconto,
            valor_seguro,
            valor_outros,
            valor_compoe_total_nota,
        })
    }
}

/// Dados sobre a tributação do produto
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

impl ProdutoTributacao {
    /// Parse do produto do item
    pub(crate) fn parse(prod: &Dom) -> Result<ProdutoTributacao, String> {
        let cfop = parsercher::search_text_from_tag_children(&prod, &Tag::new("CFOP"))
            .ok_or("Tag <CFOP> não encontrada na <prod>")?[0]
            .to_string();

        let cest = {
            if let Some(ce) = parsercher::search_text_from_tag_children(&prod, &Tag::new("CEST")) {
                Some(ce[0].to_string())
            } else {
                None
            }
        };

        let escala_relevante = {
            if let Some(er) =
                parsercher::search_text_from_tag_children(&prod, &Tag::new("indEscala"))
            {
                Some(er[0].parse::<EscalaRelevante>()?)
            } else {
                None
            }
        };

        let codigo_beneficio_fiscal = {
            if let Some(cb) = parsercher::search_text_from_tag_children(&prod, &Tag::new("cBenef"))
            {
                Some(cb[0].to_string())
            } else {
                None
            }
        };

        let codigo_excecao_ipi = {
            if let Some(ex) = parsercher::search_text_from_tag_children(&prod, &Tag::new("EXTIPI"))
            {
                Some(ex[0].to_string())
            } else {
                None
            }
        };

        let gtin = {
            if let Some(ean) =
                parsercher::search_text_from_tag_children(&prod, &Tag::new("cEANTrib"))
            {
                let gtin = ean[0].to_string();

                match gtin.to_lowercase().trim() {
                    "sem gtin" => None,
                    _ => Some(gtin),
                }
            } else {
                None
            }
        };

        let unidade = parsercher::search_text_from_tag_children(&prod, &Tag::new("uTrib"))
            .ok_or("Tag <uTrib> não encontrada na <prod>")?[0]
            .to_string();

        let quantidade = parsercher::search_text_from_tag_children(&prod, &Tag::new("qTrib"))
            .ok_or("Tag <qTrib> não encontrada na <prod>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_unitario = parsercher::search_text_from_tag_children(&prod, &Tag::new("vUnTrib"))
            .ok_or("Tag <vUnTrib> não encontrada na <prod>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        Ok(ProdutoTributacao {
            cest,
            escala_relevante,
            codigo_beneficio_fiscal,
            codigo_excecao_ipi,
            cfop,
            gtin,
            unidade,
            quantidade,
            valor_unitario,
        })
    }
}

/// Indicador de Produção em escala relevante
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum EscalaRelevante {
    Sim = 1,
    Nao = 2,
}

impl FromStr for EscalaRelevante {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().trim() {
            "s" => EscalaRelevante::Nao, // S
            _ => EscalaRelevante::Nao,   // N
        })
    }
}
