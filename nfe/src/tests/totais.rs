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
