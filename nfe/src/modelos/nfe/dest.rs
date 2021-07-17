//! Destinarário da NF-e no modelo 55

use crate::base::dest::Destinatario as DestinatarioBase;
pub use crate::base::dest::IndicadorContribuicaoIe;
pub use crate::base::endereco::Endereco;
use std::convert::TryFrom;

/// Destinarário da NF-e
pub struct Destinatario {
    pub cnpj: String,
    pub razao_social: String,
    pub endereco: Endereco,
    pub ie: Option<String>,
    pub indicador_ie: IndicadorContribuicaoIe,
}

impl TryFrom<DestinatarioBase> for Destinatario {
    type Error = String;

    fn try_from(dest: DestinatarioBase) -> Result<Self, Self::Error> {
        let razao_social = dest
            .razao_social
            .ok_or("Razão social/Nome não informado no destinatário")?;

        let endereco = dest
            .endereco
            .ok_or("Endereço não informado no destinatário")?;

        Ok(Self {
            cnpj: dest.cnpj.clone(),
            razao_social,
            endereco,
            ie: dest.ie.clone(),
            indicador_ie: dest.indicador_ie,
        })
    }
}
