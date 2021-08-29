//! Identificação da NF-e

use super::Error;
use chrono::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

mod emissao;
mod operacao;

pub use emissao::*;
pub use operacao::*;

/// Identificação da NF-e
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ModeloDocumentoFiscal {
    Nfe = 55,
    Nfce = 65,
}

/// Formato de impressão do DANFE
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
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
#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum TipoAmbiente {
    Producao = 1,
    Homologacao = 2,
}

/// Dados referentes a regeração da chave de acesso
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ComposicaoChaveAcesso {
    pub codigo: String,
    pub digito_verificador: u8,
}

impl FromStr for Identificacao {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}

impl ToString for Identificacao {
    fn to_string(&self) -> String {
        quick_xml::se::to_string(self).expect("Falha ao serializar a identificação")
    }
}

impl<'de> Deserialize<'de> for Identificacao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO: voltar a tentar usar o serde flatten

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
                codigo: ide.c_codigo.clone(),
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

impl Serialize for Identificacao {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ide = IdeContainer {
            codigo_uf: self.codigo_uf,
            numero: self.numero,
            serie: self.serie,
            modelo: self.modelo,
            codigo_municipio: self.codigo_municipio,
            formato_danfe: self.formato_danfe,
            ambiente: self.ambiente,
            c_codigo: self.chave.codigo.clone(),
            c_digito_verificador: self.chave.digito_verificador,
            o_horario: self.operacao.horario,
            o_tipo: self.operacao.tipo,
            o_destino: self.operacao.destino,
            o_natureza: self.operacao.natureza.clone(),
            o_consumidor: self.operacao.consumidor,
            o_presenca: self.operacao.presenca,
            o_intermediador: self.operacao.intermediador,
            e_horario: self.emissao.horario,
            e_tipo: self.emissao.tipo,
            e_finalidade: self.emissao.finalidade,
            e_processo: self.emissao.processo,
            e_versao_processo: self.emissao.versao_processo.clone(),
        };

        ide.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "ide")]
struct IdeContainer {
    #[serde(rename = "$unflatten=cUF")]
    pub codigo_uf: u8,
    #[serde(rename = "$unflatten=nNF")]
    pub numero: u32,
    #[serde(rename = "$unflatten=serie")]
    pub serie: u16,
    #[serde(rename = "$unflatten=mod")]
    pub modelo: ModeloDocumentoFiscal,
    #[serde(rename = "$unflatten=cMunFG")]
    pub codigo_municipio: u32,
    #[serde(rename = "$unflatten=tpImp")]
    pub formato_danfe: FormatoImpressaoDanfe,
    #[serde(rename = "$unflatten=tpAmb")]
    pub ambiente: TipoAmbiente,

    #[serde(rename = "$unflatten=cNF")]
    pub c_codigo: String,
    #[serde(rename = "$unflatten=cDV")]
    pub c_digito_verificador: u8,

    #[serde(rename = "$unflatten=dhEmi")]
    #[serde(serialize_with = "serialize_horario")]
    pub e_horario: DateTime<Utc>,
    #[serde(rename = "$unflatten=tpEmis")]
    pub e_tipo: TipoEmissao,
    #[serde(rename = "$unflatten=finNFe")]
    pub e_finalidade: FinalidadeEmissao,
    #[serde(rename = "$unflatten=procEmi")]
    pub e_processo: TipoProcessoEmissao,
    #[serde(rename = "$unflatten=verProc")]
    pub e_versao_processo: String,

    #[serde(rename = "$unflatten=dhSaiEnt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_horario_op")]
    pub o_horario: Option<DateTime<Utc>>,
    #[serde(rename = "$unflatten=tpNF")]
    pub o_tipo: TipoOperacao,
    #[serde(rename = "$unflatten=idDest")]
    pub o_destino: DestinoOperacao,
    #[serde(rename = "$unflatten=natOp")]
    pub o_natureza: String,
    #[serde(rename = "$unflatten=indFinal")]
    pub o_consumidor: TipoConsumidor,
    #[serde(rename = "$unflatten=indPres")]
    pub o_presenca: TipoPresencaComprador,
    #[serde(rename = "$unflatten=indIntermed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_intermediador: Option<TipoIntermediador>,
}

fn serialize_horario<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.to_rfc3339())
}

fn serialize_horario_op<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serialize_horario(&date.unwrap(), serializer)
}
