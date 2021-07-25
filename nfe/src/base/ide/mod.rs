//! Identificação da NF-e

use chrono::prelude::*;
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use std::str::FromStr;

mod emissao;
mod operacao;

pub use emissao::*;
pub use operacao::*;

/// Identificação da NF-e
#[derive(Debug, PartialEq)]
pub struct Identificacao {
    pub codigo_uf: u8,
    pub chave: ComposicaoChaveAcesso,
    pub numero: u32,
    pub serie: u16,
    pub modelo: ModeloDocumentoFiscal,
    pub emissao: Emissao,
    pub operacao: Operacao,
    pub codigo_municipio: u32,
    pub formato_danfe: FormatoImpressaoDanfe,
    pub ambiente: TipoAmbiente,
}

/// Modelo do documento fiscal: NF-e ou NFC-e
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum ModeloDocumentoFiscal {
    Nfe = 55,
    Nfce = 65,
}

/// Formato de impressão do DANFE
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum FormatoImpressaoDanfe {
    SemGeracao = 0,
    NormalRetrato = 1,
    NormalPaisagem = 2,
    Simplificado = 3,
    Nfce = 4,
    NfceMensagemEletronica = 5,
}

/// Tipo do ambiente da NF
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum TipoAmbiente {
    Producao = 1,
    Homologacao = 2,
}

/// Dados referentes a regeração da chave de acesso
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ComposicaoChaveAcesso {
    pub codigo: u32,
    pub digito_verificador: u8,
}

impl FromStr for Identificacao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s).map_err(|e| e.to_string())
    }
}

impl<'de> Deserialize<'de> for Identificacao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO: voltar a tentar usar o serde flatten

        #[derive(Deserialize)]
        struct IdeContainer {
            #[serde(rename = "cUF")]
            pub codigo_uf: u8,
            #[serde(rename = "nNF")]
            pub numero: u32,
            #[serde(rename = "serie")]
            pub serie: u16,
            #[serde(rename = "mod")]
            pub modelo: ModeloDocumentoFiscal,
            #[serde(rename = "cMunFG")]
            pub codigo_municipio: u32,
            #[serde(rename = "tpImp")]
            pub formato_danfe: FormatoImpressaoDanfe,
            #[serde(rename = "tpAmb")]
            pub ambiente: TipoAmbiente,

            #[serde(rename = "cNF")]
            pub c_codigo: u32,
            #[serde(rename = "cDV")]
            pub c_digito_verificador: u8,

            #[serde(rename = "dhEmi")]
            pub e_horario: DateTime<Utc>,
            #[serde(rename = "tpEmis")]
            pub e_tipo: TipoEmissao,
            #[serde(rename = "finNFe")]
            pub e_finalidade: FinalidadeEmissao,
            #[serde(rename = "procEmi")]
            pub e_processo: TipoProcessoEmissao,
            #[serde(rename = "verProc")]
            pub e_versao_processo: String,

            #[serde(rename = "dhSaiEnt")]
            pub o_horario: Option<DateTime<Utc>>,
            #[serde(rename = "tpNF")]
            pub o_tipo: TipoOperacao,
            #[serde(rename = "idDest")]
            pub o_destino: DestinoOperacao,
            #[serde(rename = "natOp")]
            pub o_natureza: String,
            #[serde(rename = "indFinal")]
            pub o_consumidor: TipoConsumidor,
            #[serde(rename = "indPres")]
            pub o_presenca: TipoPresencaComprador,
            #[serde(rename = "indIntermed")]
            pub o_intermediador: Option<TipoIntermediador>,
        }

        let ide = IdeContainer::deserialize(deserializer)?;

        Ok(Self {
            codigo_uf: ide.codigo_uf,
            numero: ide.numero,
            serie: ide.serie,
            modelo: ide.modelo,
            codigo_municipio: ide.codigo_municipio,
            formato_danfe: ide.formato_danfe,
            ambiente: ide.ambiente,
            chave: ComposicaoChaveAcesso {
                codigo: ide.c_codigo,
                digito_verificador: ide.c_digito_verificador,
            },
            operacao: Operacao {
                horario: ide.o_horario,
                tipo: ide.o_tipo,
                destino: ide.o_destino,
                natureza: ide.o_natureza,
                consumidor: ide.o_consumidor,
                presenca: ide.o_presenca,
                intermediador: ide.o_intermediador,
            },
            emissao: Emissao {
                horario: ide.e_horario,
                tipo: ide.e_tipo,
                finalidade: ide.e_finalidade,
                processo: ide.e_processo,
                versao_processo: ide.e_versao_processo,
            },
        })
    }
}
