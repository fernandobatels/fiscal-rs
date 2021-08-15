//! Destinarário da NF-e no modelo 55

use super::Error;
use crate::base::dest::Destinatario as DestinatarioBase;
pub use crate::base::dest::IndicadorContribuicaoIe;
pub use crate::base::endereco::Endereco;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

/// Destinatário da NF-e
pub struct Destinatario {
    pub cnpj: String,
    pub razao_social: String,
    pub endereco: Endereco,
    pub ie: Option<String>,
    pub indicador_ie: IndicadorContribuicaoIe,
}

impl TryFrom<DestinatarioBase> for Destinatario {
    type Error = Error;

    fn try_from(dest: DestinatarioBase) -> Result<Self, Self::Error> {
        let razao_social = dest.razao_social.ok_or_else(|| {
            Error::DestinatarioInvalido("Razão social/Nome não informado".to_string())
        })?;

        let endereco = dest
            .endereco
            .ok_or_else(|| Error::DestinatarioInvalido("Endereço não informado".to_string()))?;

        Ok(Self {
            cnpj: dest.cnpj.clone(),
            razao_social,
            endereco,
            ie: dest.ie.clone(),
            indicador_ie: dest.indicador_ie,
        })
    }
}

impl From<&Destinatario> for DestinatarioBase {
    fn from(dest: &Destinatario) -> Self {
        Self {
            cnpj: dest.cnpj.clone(),
            razao_social: Some(dest.razao_social.clone()),
            endereco: Some(dest.endereco.clone()),
            ie: dest.ie.clone(),
            indicador_ie: dest.indicador_ie,
        }
    }
}

impl FromStr for Destinatario {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let base = s.parse::<DestinatarioBase>()?;

        base.try_into()
    }
}

impl ToString for Destinatario {
    fn to_string(&self) -> String {
        let base: DestinatarioBase = self.into();

        base.to_string()
    }
}
