//! Testes da totalização das notas

use std::convert::TryFrom;
use std::fs::File;

use crate::base::Nfe as NfeBase;
use crate::*;

#[test]
fn apenas_valores_dos_produtos() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let totais = Nfe::try_from(f)?.totais;

    assert_eq!(0.0, totais.valor_base_calculo);
    assert_eq!(0.0, totais.valor_icms);
    assert_eq!(500.0, totais.valor_produtos);
    assert_eq!(0.0, totais.valor_frete);
    assert_eq!(0.0, totais.valor_seguro);
    assert_eq!(0.0, totais.valor_desconto);
    assert_eq!(0.0, totais.valor_pis);
    assert_eq!(0.0, totais.valor_outros);
    assert_eq!(0.0, totais.valor_cofins);
    assert_eq!(500.0, totais.valor_total);
    assert_eq!(0.0, totais.valor_aproximado_tributos);

    Ok(())
}

#[test]
fn produtos_com_pis_cofins() -> Result<(), String> {
    let f = File::open("xmls/nfce_layout4.xml").map_err(|e| e.to_string())?;
    let totais = NfeBase::try_from(f)?.totais;

    assert_eq!(0.0, totais.valor_base_calculo);
    assert_eq!(0.0, totais.valor_icms);
    assert_eq!(150.0, totais.valor_produtos);
    assert_eq!(0.0, totais.valor_frete);
    assert_eq!(0.0, totais.valor_seguro);
    assert_eq!(0.0, totais.valor_desconto);
    assert_eq!(0.89, totais.valor_pis);
    assert_eq!(0.0, totais.valor_outros);
    assert_eq!(4.09, totais.valor_cofins);
    assert_eq!(150.0, totais.valor_total);
    assert_eq!(35.75, totais.valor_aproximado_tributos);

    Ok(())
}

#[test]
fn manual_produtos_com_pis_cofins() -> Result<(), String> {
    let xml = "<total>
            <ICMSTot>
                <vBC>0.00</vBC>
                <vICMS>0.00</vICMS>
                <vICMSDeson>0.00</vICMSDeson>
                <vFCP>0.00</vFCP>
                <vBCST>0.00</vBCST>
                <vST>0.00</vST>
                <vFCPST>0.00</vFCPST>
                <vFCPSTRet>0.00</vFCPSTRet>
                <vProd>150.00</vProd>
                <vFrete>0.00</vFrete>
                <vSeg>0.00</vSeg>
                <vDesc>0.00</vDesc>
                <vII>0.00</vII>
                <vIPI>0.00</vIPI>
                <vIPIDevol>0.00</vIPIDevol>
                <vPIS>0.89</vPIS>
                <vCOFINS>4.09</vCOFINS>
                <vOutro>0.00</vOutro>
                <vNF>150.00</vNF>
                <vTotTrib>35.75</vTotTrib>
            </ICMSTot>
        </total>";

    let totais = xml.parse::<Totalizacao>()?;

    assert_eq!(0.0, totais.valor_base_calculo);
    assert_eq!(0.0, totais.valor_icms);
    assert_eq!(150.0, totais.valor_produtos);
    assert_eq!(0.0, totais.valor_frete);
    assert_eq!(0.0, totais.valor_seguro);
    assert_eq!(0.0, totais.valor_desconto);
    assert_eq!(0.89, totais.valor_pis);
    assert_eq!(0.0, totais.valor_outros);
    assert_eq!(4.09, totais.valor_cofins);
    assert_eq!(150.0, totais.valor_total);
    assert_eq!(35.75, totais.valor_aproximado_tributos);

    Ok(())
}
