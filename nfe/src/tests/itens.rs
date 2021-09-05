//! Testes dos itens/produtos da nf

use std::convert::TryFrom;
use std::fs::File;

use crate::base::Nfe as NfeBase;
use crate::*;

#[test]
fn from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let itens = Nfe::try_from(f).map_err(|e| e.to_string())?.itens;

    assert_eq!(1, itens.len());

    let item = &itens[0];

    assert_eq!(1, item.numero);

    Ok(())
}

#[test]
fn manual() -> Result<(), Error> {
    let xml = "
        <det nItem=\"1\">
            <prod>
                <cProd>11007</cProd>
                <cEAN>SEM GTIN</cEAN>
                <xProd>UM PRODUTO TESTE QUALQUER</xProd>
                <NCM>64011000</NCM>
                <CEST>1234567</CEST>
                <CFOP>6101</CFOP>
                <uCom>UN</uCom>
                <qCom>10.0000</qCom>
                <vUnCom>50</vUnCom>
                <vProd>500.00</vProd>
                <cEANTrib>SEM GTIN</cEANTrib>
                <uTrib>UN</uTrib>
                <qTrib>10.0000</qTrib>
                <vUnTrib>50.0000</vUnTrib>
                <indTot>1</indTot>
            </prod>
            <imposto>
                <vTotTrib>0.00</vTotTrib>
            </imposto>
       </det>
    ";

    let item = xml.parse::<Item>()?;

    assert_eq!(1, item.numero);

    Ok(())
}

#[test]
fn produto_from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let itens = Nfe::try_from(f).map_err(|e| e.to_string())?.itens;

    assert_eq!(1, itens.len());

    let produto = &itens[0].produto;

    assert_eq!("11007", produto.codigo);
    assert_eq!(None, produto.gtin);
    assert_eq!("UM PRODUTO TESTE QUALQUER", produto.descricao);
    assert_eq!("64011000", produto.ncm);
    assert_eq!(Some("1234567".to_string()), produto.tributacao.cest);
    assert_eq!(None, produto.tributacao.escala_relevante);
    assert_eq!(None, produto.fabricante_cnpj);
    assert_eq!(None, produto.tributacao.codigo_beneficio_fiscal);
    assert_eq!(None, produto.tributacao.codigo_excecao_ipi);
    assert_eq!("6101", produto.tributacao.cfop);
    assert_eq!("UN", produto.unidade);
    assert_eq!(10.00, produto.quantidade);
    assert_eq!(50.00, produto.valor_unitario);
    assert_eq!(None, produto.tributacao.gtin);
    assert_eq!("UN", produto.tributacao.unidade);
    assert_eq!(10.00, produto.tributacao.quantidade);
    assert_eq!(50.00, produto.tributacao.valor_unitario);
    assert_eq!(500.00, produto.valor_bruto);
    assert_eq!(None, produto.valor_frete);
    assert_eq!(None, produto.valor_seguro);
    assert_eq!(None, produto.valor_desconto);
    assert_eq!(None, produto.valor_outros);
    assert_eq!(true, produto.valor_compoe_total_nota);

    Ok(())
}

#[test]
fn produto_manual() -> Result<(), Error> {
    let xml = "
        <prod>
            <cProd>11007</cProd>
            <cEAN>SEM GTIN</cEAN>
            <xProd>UM PRODUTO TESTE QUALQUER</xProd>
            <NCM>64011000</NCM>
            <CEST>1234567</CEST>
            <CFOP>6101</CFOP>
            <uCom>UN</uCom>
            <qCom>10.0000</qCom>
            <vUnCom>50</vUnCom>
            <vProd>500.00</vProd>
            <cEANTrib>SEM GTIN</cEANTrib>
            <uTrib>UN</uTrib>
            <qTrib>10.0000</qTrib>
            <vUnTrib>50.0000</vUnTrib>
            <indTot>1</indTot>
        </prod>
    ";

    let produto = xml.parse::<Produto>()?;

    assert_eq!("11007", produto.codigo);
    assert_eq!(None, produto.gtin);
    assert_eq!("UM PRODUTO TESTE QUALQUER", produto.descricao);
    assert_eq!("64011000", produto.ncm);
    assert_eq!(Some("1234567".to_string()), produto.tributacao.cest);
    assert_eq!(None, produto.tributacao.escala_relevante);
    assert_eq!(None, produto.fabricante_cnpj);
    assert_eq!(None, produto.tributacao.codigo_beneficio_fiscal);
    assert_eq!(None, produto.tributacao.codigo_excecao_ipi);
    assert_eq!("6101", produto.tributacao.cfop);
    assert_eq!("UN", produto.unidade);
    assert_eq!(10.00, produto.quantidade);
    assert_eq!(50.00, produto.valor_unitario);
    assert_eq!(None, produto.tributacao.gtin);
    assert_eq!("UN", produto.tributacao.unidade);
    assert_eq!(10.00, produto.tributacao.quantidade);
    assert_eq!(50.00, produto.tributacao.valor_unitario);
    assert_eq!(500.00, produto.valor_bruto);
    assert_eq!(None, produto.valor_frete);
    assert_eq!(None, produto.valor_seguro);
    assert_eq!(None, produto.valor_desconto);
    assert_eq!(None, produto.valor_outros);
    assert_eq!(true, produto.valor_compoe_total_nota);

    Ok(())
}

#[test]
fn produtos() -> Result<(), String> {
    let f = File::open("xmls/nfce_layout4.xml").map_err(|e| e.to_string())?;
    let itens = NfeBase::try_from(f).map_err(|e| e.to_string())?.itens;

    assert_eq!(2, itens.len());

    let produto = &itens[0].produto;

    assert_eq!("10015300336", produto.codigo);
    assert_eq!(Some("7893049207584".to_string()), produto.gtin);
    assert_eq!("(153 - C2075) -CILINDRO MESTRE DUPLO UN", produto.descricao);
    assert_eq!("87083090", produto.ncm);
    assert_eq!(None, produto.tributacao.cest);
    assert_eq!(None, produto.tributacao.escala_relevante);
    assert_eq!(None, produto.fabricante_cnpj);
    assert_eq!(None, produto.tributacao.codigo_beneficio_fiscal);
    assert_eq!(None, produto.tributacao.codigo_excecao_ipi);
    assert_eq!("5405", produto.tributacao.cfop);
    assert_eq!("UN", produto.unidade);
    assert_eq!(1.00, produto.quantidade);
    assert_eq!(96.22, produto.valor_unitario);
    assert_eq!(Some("7893049207584".to_string()), produto.tributacao.gtin);
    assert_eq!("UN", produto.tributacao.unidade);
    assert_eq!(1.00, produto.tributacao.quantidade);
    assert_eq!(96.22, produto.tributacao.valor_unitario);
    assert_eq!(96.22, produto.valor_bruto);
    assert_eq!(None, produto.valor_frete);
    assert_eq!(None, produto.valor_seguro);
    assert_eq!(None, produto.valor_desconto);
    assert_eq!(None, produto.valor_outros);
    assert_eq!(true, produto.valor_compoe_total_nota);

    let produto = &itens[1].produto;

    assert_eq!("10029200332", produto.codigo);
    assert_eq!(None, produto.gtin);
    assert_eq!(
        "(292 - BAH0031D) -ROLAMENTO RODA DIANTEIRO SEM ABS UN",
        produto.descricao
    );
    assert_eq!("84821090", produto.ncm);
    assert_eq!(None, produto.tributacao.cest);
    assert_eq!(None, produto.tributacao.escala_relevante);
    assert_eq!(None, produto.fabricante_cnpj);
    assert_eq!(None, produto.tributacao.codigo_beneficio_fiscal);
    assert_eq!(None, produto.tributacao.codigo_excecao_ipi);
    assert_eq!("5405", produto.tributacao.cfop);
    assert_eq!("UN", produto.unidade);
    assert_eq!(1.00, produto.quantidade);
    assert_eq!(53.78, produto.valor_unitario);
    assert_eq!(None, produto.tributacao.gtin);
    assert_eq!("UN", produto.tributacao.unidade);
    assert_eq!(1.00, produto.tributacao.quantidade);
    assert_eq!(53.78, produto.tributacao.valor_unitario);
    assert_eq!(53.78, produto.valor_bruto);
    assert_eq!(None, produto.valor_frete);
    assert_eq!(None, produto.valor_seguro);
    assert_eq!(None, produto.valor_desconto);
    assert_eq!(None, produto.valor_outros);
    assert_eq!(true, produto.valor_compoe_total_nota);

    Ok(())
}

#[test]
fn imposto_from_instance() -> Result<(), String> {
    let f = File::open("xmls/nfe_layout4.xml").map_err(|e| e.to_string())?;
    let itens = Nfe::try_from(f).map_err(|e| e.to_string())?.itens;

    assert_eq!(1, itens.len());

    let imposto = &itens[0].imposto;

    assert_eq!(Some(0.0), imposto.valor_aproximado);
    assert_eq!(
        Some(GrupoIcms::IcmsSn202(GrupoIcmsSn202 {
            origem: OrigemMercadoria::Nacional,
            aliquota: 0.0,
            valor: 0.0,
            valor_base_calculo: 0.0,
            base_calculo: ModalidadeBaseCalculoIcmsSt::MargemValorAgregado,
            codigo_situacao: "202".to_string()
        })),
        imposto.icms
    );
    assert_eq!(
        Some(GrupoPis::PisOutr(GrupoPisOutr {
            aliquota: 0.0,
            valor_base_calculo: 0.0,
            codigo_situacao: "49".to_string()
        })),
        imposto.pis
    );
    assert_eq!(
        Some(GrupoCofins::CofinsOutr(GrupoCofinsOutr {
            aliquota: 0.0,
            valor_base_calculo: 0.0,
            codigo_situacao: "49".to_string()
        })),
        imposto.cofins
    );

    Ok(())
}

#[test]
fn imposto_manual() -> Result<(), Error> {
    let xml = "
        <imposto>
            <vTotTrib>0.00</vTotTrib>
            <ICMS>
                <ICMSSN202>
                    <orig>0</orig>
                    <CSOSN>202</CSOSN>
                    <modBCST>4</modBCST>
                    <vBCST>0.00</vBCST>
                    <pICMSST>0.0000</pICMSST>
                    <vICMSST>0.00</vICMSST>
                </ICMSSN202>
            </ICMS>
            <PIS>
                <PISOutr>
                    <CST>49</CST>
                    <vBC>0.00</vBC>
                    <pPIS>0.0000</pPIS>
                    <vPIS>0.00</vPIS>
                </PISOutr>
            </PIS>
            <COFINS>
                <COFINSOutr>
                    <CST>49</CST>
                    <vBC>0.00</vBC>
                    <pCOFINS>0.0000</pCOFINS>
                    <vCOFINS>0.00</vCOFINS>
                </COFINSOutr>
            </COFINS>
        </imposto>
    ";

    let imposto = xml.parse::<Imposto>()?;

    assert_eq!(Some(0.0), imposto.valor_aproximado);
    assert_eq!(
        Some(GrupoIcms::IcmsSn202(GrupoIcmsSn202 {
            origem: OrigemMercadoria::Nacional,
            aliquota: 0.0,
            valor: 0.0,
            valor_base_calculo: 0.0,
            base_calculo: ModalidadeBaseCalculoIcmsSt::MargemValorAgregado,
            codigo_situacao: "202".to_string()
        })),
        imposto.icms
    );
    assert_eq!(
        Some(GrupoPis::PisOutr(GrupoPisOutr {
            aliquota: 0.0,
            valor_base_calculo: 0.0,
            codigo_situacao: "49".to_string()
        })),
        imposto.pis
    );
    assert_eq!(
        Some(GrupoCofins::CofinsOutr(GrupoCofinsOutr {
            aliquota: 0.0,
            valor_base_calculo: 0.0,
            codigo_situacao: "49".to_string()
        })),
        imposto.cofins
    );

    Ok(())
}

#[test]
fn impostos() -> Result<(), String> {
    let f = File::open("xmls/nfce_layout4.xml").map_err(|e| e.to_string())?;
    let itens = NfeBase::try_from(f).map_err(|e| e.to_string())?.itens;

    assert_eq!(2, itens.len());

    let imposto = &itens[0].imposto;

    assert_eq!(Some(17.32), imposto.valor_aproximado);
    assert_eq!(
        Some(GrupoIcms::Icms60(GrupoIcms60 {
            origem: OrigemMercadoria::Nacional,
            aliquota: 0.0,
            valor: 0.0,
            valor_base_calculo: 0.0,
        })),
        imposto.icms
    );
    assert_eq!(
        Some(GrupoPis::PisNt(GrupoPisNt {
            codigo_situacao: "04".to_string()
        })),
        imposto.pis
    );
    assert_eq!(
        Some(GrupoCofins::CofinsNt(GrupoCofinsNt {
            codigo_situacao: "04".to_string()
        })),
        imposto.cofins
    );

    let imposto = &itens[1].imposto;

    assert_eq!(Some(18.43), imposto.valor_aproximado);
    assert_eq!(
        Some(GrupoIcms::Icms60(GrupoIcms60 {
            origem: OrigemMercadoria::Nacional,
            aliquota: 0.0,
            valor: 0.0,
            valor_base_calculo: 0.0,
        })),
        imposto.icms
    );
    assert_eq!(
        Some(GrupoPis::PisAliq(GrupoPisAliq {
            valor: 0.89,
            aliquota: 1.65,
            valor_base_calculo: 53.78,
            codigo_situacao: "01".to_string()
        })),
        imposto.pis
    );
    assert_eq!(
        Some(GrupoCofins::CofinsAliq(GrupoCofinsAliq {
            valor: 4.09,
            aliquota: 7.6,
            valor_base_calculo: 53.78,
            codigo_situacao: "01".to_string()
        })),
        imposto.cofins
    );

    Ok(())
}

#[test]
fn produto_to_string() -> Result<(), Error> {
    let mut xml_original = "<prod>
        <cProd>11007</cProd>
        <cEAN>SEM GTIN</cEAN>
        <xProd>UM PRODUTO TESTE QUALQUER</xProd>
        <NCM>64011000</NCM>
        <uCom>UN</uCom>
        <qCom>10</qCom>
        <vUnCom>50</vUnCom>
        <vProd>500</vProd>
        <indTot>1</indTot>
        <CEST>1234567</CEST>
        <CFOP>6101</CFOP>
        <cEANTrib>SEM GTIN</cEANTrib>
        <uTrib>UN</uTrib>
        <qTrib>10</qTrib>
        <vUnTrib>50</vUnTrib>
    </prod>"
        .to_string();
    xml_original.retain(|c| c != '\n' && c != ' ');

    let produto = xml_original.parse::<Produto>()?;
    let xml_novo = produto.to_string();

    assert_eq!(xml_original, xml_novo);

    Ok(())
}

#[test]
fn imposto_to_string() -> Result<(), Error> {
    let mut xml_original = "<imposto>
            <vTotTrib>0</vTotTrib>
            <ICMS>
                <ICMSSN202>
                    <orig>0</orig>
                    <CSOSN>202</CSOSN>
                    <modBCST>4</modBCST>
                    <vBCST>0</vBCST>
                    <pICMSST>0</pICMSST>
                    <vICMSST>0</vICMSST>
                </ICMSSN202>
            </ICMS>
            <PIS>
                <PISOutr>
                    <CST>49</CST>
                    <vBC>0</vBC>
                    <pPIS>0</pPIS>
                </PISOutr>
            </PIS>
            <COFINS>
                <COFINSOutr>
                    <CST>49</CST>
                    <vBC>0</vBC>
                    <pCOFINS>0</pCOFINS>
                </COFINSOutr>
            </COFINS>
        </imposto>"
        .to_string();
    xml_original.retain(|c| c != '\n' && c != ' ');

    let imposto = xml_original.parse::<Imposto>()?;
    let xml_novo = imposto.to_string();

    assert_eq!(xml_original, xml_novo);

    Ok(())
}

#[test]
fn to_string() -> Result<(), Error> {
    let mut xml_original = "
        <det nItem=\"1\">
            <prod>
                <cProd>11007</cProd>
                <cEAN>SEM GTIN</cEAN>
                <xProd>UM PRODUTO TESTE QUALQUER</xProd>
                <NCM>64011000</NCM>
                <uCom>UN</uCom>
                <qCom>10</qCom>
                <vUnCom>50</vUnCom>
                <vProd>500</vProd>
                <indTot>1</indTot>
                <CEST>1234567</CEST>
                <CFOP>6101</CFOP>
                <cEANTrib>SEM GTIN</cEANTrib>
                <uTrib>UN</uTrib>
                <qTrib>10</qTrib>
                <vUnTrib>50</vUnTrib>
            </prod>
            <imposto>
                <vTotTrib>0</vTotTrib>
            </imposto>
       </det>"
        .to_string();
    xml_original.retain(|c| c != '\n' && c != ' ');
    xml_original = xml_original.replace("detnItem", "det nItem");

    let item = xml_original.parse::<Item>()?;
    let xml_novo = item.to_string();

    assert_eq!(xml_original, xml_novo);

    Ok(())
}
