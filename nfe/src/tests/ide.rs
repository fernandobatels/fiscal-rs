//! Testes da tag <ide>

use std::convert::TryFrom;
use std::fs::File;

use crate::*;

#[test]
fn basico() -> Result<(), String> {

    let f = File::open("xmls/nfe_layout4.xml")
        .map_err(|e| e.to_string())?;
    let nfe = Nfe::try_from(f)?;

    assert_eq!(43, nfe.ide.codigo_uf);
    assert_eq!(1030, nfe.ide.codigo_chave);
    assert_eq!("Venda de producao do estabelecimento", nfe.ide.natureza_operacao);
    assert_eq!(1, nfe.ide.serie);
    assert_eq!(26, nfe.ide.numero);
    assert_eq!(ModeloDocumentoFiscal::Nfe, nfe.ide.modelo);

    Ok(())
}
