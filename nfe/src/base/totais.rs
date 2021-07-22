//! Totalização dos produtos e serviços

use parsercher::dom::*;

/// Totalização da nota fiscal
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
    pub valor_aproximado_tributos: f32
}

impl Totalizacao {
    /// Parse da seção <total>
    pub(crate) fn parse(xml: &Dom) -> Result<Totalizacao, String> {

        let mut t_total = Dom::new(DomType::Tag);
        t_total.set_tag(Tag::new("total"));
        let total = parsercher::search_dom(&xml, &t_total).ok_or("Tag <total> não encontrada")?;

        let mut t_icms_total = Dom::new(DomType::Tag);
        t_icms_total.set_tag(Tag::new("ICMSTot"));
        let icms_total = parsercher::search_dom(&total, &t_icms_total).ok_or("Tag <ICMSTot> não encontrada na <total>")?;

        let valor_base_calculo = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vBC"))
            .ok_or("Tag <vBC> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_icms = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vICMS"))
            .ok_or("Tag <vICMS> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_produtos = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vProd"))
            .ok_or("Tag <vProd> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_frete = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vFrete"))
            .ok_or("Tag <vFrete> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_seguro = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vSeg"))
            .ok_or("Tag <vSeg> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_desconto = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vDesc"))
            .ok_or("Tag <vDesc> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_pis = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vPIS"))
            .ok_or("Tag <vPIS> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_cofins = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vCOFINS"))
            .ok_or("Tag <vCOFINS> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_outros = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vOutro"))
            .ok_or("Tag <vOutro> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_total = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vNF"))
            .ok_or("Tag <vNF> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        let valor_aproximado_tributos = parsercher::search_text_from_tag_children(&icms_total, &Tag::new("vTotTrib"))
            .ok_or("Tag <vTotTrib> não encontrada na <ICMSTot>")?[0]
            .parse::<f32>()
            .map_err(|e| e.to_string())?;

        Ok(Totalizacao {
            valor_base_calculo,
            valor_icms,
            valor_produtos,
            valor_frete,
            valor_seguro,
            valor_desconto,
            valor_pis,
            valor_cofins,
            valor_outros,
            valor_total,
            valor_aproximado_tributos,
        })
    }
}
