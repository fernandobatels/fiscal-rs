//! Dados da operação da NF-e

use chrono::prelude::*;
use parsercher::dom::*;
use std::str::FromStr;

/// Dados referentes a operação da nota
pub struct Operacao {
    pub horario: Option<DateTime<Utc>>,
    pub tipo: TipoOperacao,
    pub destino: DestinoOperacao,
    pub natureza: String,
    pub consumidor: TipoConsumidor,
    pub presenca: TipoPresencaComprador,
    pub intermediador: Option<TipoIntermediador>,
}

impl Operacao {
    /// Parse dos campos da tag <ide> relacionados a operação
    pub fn parse(ide: &Dom) -> Result<Operacao, String> {
        let natureza = parsercher::search_text_from_tag_children(&ide, &Tag::new("natOp"))
            .ok_or("Tag <natOp> não encontrada na <ide>")?[0]
            .to_string();

        let tipo = parsercher::search_text_from_tag_children(&ide, &Tag::new("tpNF"))
            .ok_or("Tag <tpNF> não encontrada na <ide>")?[0]
            .parse::<TipoOperacao>()?;

        let destino = parsercher::search_text_from_tag_children(&ide, &Tag::new("idDest"))
            .ok_or("Tag <idDest> não encontrada na <ide>")?[0]
            .parse::<DestinoOperacao>()?;

        let horario = {
            if let Some(dt) = parsercher::search_text_from_tag_children(&ide, &Tag::new("dhSaiEnt"))
            {
                Some(dt[0].parse::<DateTime<Utc>>().map_err(|e| e.to_string())?)
            } else {
                None
            }
        };

        let consumidor = parsercher::search_text_from_tag_children(&ide, &Tag::new("indFinal"))
            .ok_or("Tag <indFinal> não encontrada na <ide>")?[0]
            .parse::<TipoConsumidor>()?;

        let presenca = parsercher::search_text_from_tag_children(&ide, &Tag::new("indPres"))
            .ok_or("Tag <indPres> não encontrada na <ide>")?[0]
            .parse::<TipoPresencaComprador>()?;

        let intermediador = {
            if let Some(dt) =
                parsercher::search_text_from_tag_children(&ide, &Tag::new("indIntermed"))
            {
                Some(dt[0].parse::<TipoIntermediador>()?)
            } else {
                None
            }
        };

        Ok(Operacao {
            natureza,
            tipo,
            destino,
            horario,
            consumidor,
            presenca,
            intermediador,
        })
    }
}

/// Tipo de operação da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoOperacao {
    Entrada = 0,
    Saida = 1,
}

/// Destino da operação da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DestinoOperacao {
    Interna = 0,
    Interestadual = 1,
    ComExterior = 2,
}

/// Tipo do consumidor da NF-e
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoConsumidor {
    Normal = 0,
    Final = 1,
}

/// Tipo da presença do comprador
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoPresencaComprador {
    /// Não se aplica. Ex.: Nota complementar ou de ajuste
    NaoSeAplica = 0,
    /// Operação presencial
    Presencial = 1,
    /// Operação não presencial, via internet
    ViaInternel = 2,
    /// Operação não presencial, via teleatendimento
    ViaTeleatendimento = 3,
    /// NFC-e em operação com entrega a domicílio
    NfceEmDomicilio = 4,
    /// Operação presencial, fora do estabelecimento
    PresencialForaDoEstabelecimento = 5,
    /// Operação não presencial
    Outros = 9,
}

/// Tipo do intermediador
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoIntermediador {
    /// Operação sem intermediador (em site ou plataforma própria)
    SemIntermediador = 0,
    /// Operação em site ou plataforma de terceiros (intermediadores/marketplace)
    EmSiteDeTerceiros = 1,
}

impl FromStr for TipoOperacao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => TipoOperacao::Saida,
            _ => TipoOperacao::Entrada, // 0
        })
    }
}

impl FromStr for DestinoOperacao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "3" => DestinoOperacao::ComExterior,
            "2" => DestinoOperacao::Interestadual,
            _ => DestinoOperacao::Interna, // 1
        })
    }
}

impl FromStr for TipoConsumidor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "2" => TipoConsumidor::Final,
            _ => TipoConsumidor::Normal, // 1
        })
    }
}

impl FromStr for TipoPresencaComprador {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "9" => TipoPresencaComprador::Outros,
            "5" => TipoPresencaComprador::PresencialForaDoEstabelecimento,
            "4" => TipoPresencaComprador::NfceEmDomicilio,
            "3" => TipoPresencaComprador::ViaTeleatendimento,
            "2" => TipoPresencaComprador::ViaInternel,
            "1" => TipoPresencaComprador::Presencial,
            _ => TipoPresencaComprador::NaoSeAplica, // 0
        })
    }
}

impl FromStr for TipoIntermediador {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => TipoIntermediador::EmSiteDeTerceiros,
            _ => TipoIntermediador::SemIntermediador, // 0
        })
    }
}
