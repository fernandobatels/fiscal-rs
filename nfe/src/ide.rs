//! Identificação da NF-e

use std::str::FromStr;
use parsercher::dom::*;
use chrono::prelude::*;

/// Identificação da NF-e
pub struct Identificacao {
    pub codigo_uf: u8,
    pub codigo_chave: u32,
    pub numero: u32,
    pub serie: u16,
    pub modelo: ModeloDocumentoFiscal,
    pub emissao: Emissao,
    pub operacao: Operacao,
    pub codigo_municipio: u32,
    pub formato_danfe: FormatoImpressaoDanfe,
    pub ambiente: TipoAmbiente
}

/// Modelo do documento fiscal: NF-e ou NFC-e
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ModeloDocumentoFiscal {
    Nfe = 55,
    Nfce = 65
}

/// Tipo de operação da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoOperacao {
    Entrada = 0,
    Saida = 1
}

/// Destino da operação da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DestinoOperacao {
    Interna = 0,
    Interestadual = 1,
    ComExterior = 2
}

/// Tipo da emissão da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoEmissao {
    /// Emissão normal (não em contingência)
    Normal = 1,
    /// Contingência FS-IA, com impressão do DANFE em Formulário de Segurança - Impressor Autônomo
    ContigenciaFsIa = 2,
    /// Contingência SCAN (Sistema de Contingência do Ambiente Nacional)
    ContingenciaScan = 3,
    /// Contingência EPEC (Evento Prévio da Emissão em Contingência)
    ContigenciaEpec = 4,
    /// Contingência FS-DA, com impressão do DANFE em Formulário de Segurança - Documento Auxiliar
    ContigenciaFsDa = 5,
    /// Contingência SVC-AN (SEFAZ Virtual de Contingência do AN)
    ContigenciaSvcAn = 6,
    /// Contingência SVC-RS (SEFAZ Virtual de Contingência do RS)
    ContigenciaSvcRs = 7,
    /// Contingência off-line da NFC-e
    ContigenciaOfflineNfce = 9
}

/// Formato de impressão do DANFE
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FormatoImpressaoDanfe {
    SemGeracao = 0,
    NormalRetrato = 1,
    NormalPaisagem = 2,
    Simplificado = 3,
    Nfce = 4,
    NfceMensagemEletronica = 5
}

/// Tipo do ambiente da NF
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoAmbiente {
    Producao = 1,
    Homologacao = 2,
}

/// Finalidade da emissão da nota
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FinalidadeEmissao {
    Normal = 1,
    Complementar = 2,
    Ajuste = 3,
    Devolucao = 4
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
    Outros = 9
}

/// Tipo do intermediador
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoIntermediador {
    /// Operação sem intermediador (em site ou plataforma própria)
    SemIntermediador = 0,
    /// Operação em site ou plataforma de terceiros (intermediadores/marketplace)
    EmSiteDeTerceiros = 1,
}

/// Tipo do processo de emissão
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TipoProcessoEmissao {
    /// Emissão de NF-e com aplicativo do contribuinte
    ViaAplicativoDoContribuinte = 0,
    /// Emissão de NF-e avulsa pelo Fisco
    AvulsaPeloFisco = 1,
    /// Emissão de NF-e avulsa, pelo contribuinte com seu certificado digital, através do site do Fisco
    AvulsaPeloContribuinte = 2,
    /// Emissão NF-e pelo contribuinte com aplicativo fornecido pelo Fisco
    ViaAplicativoDoFisco = 3,
}

/// Dados referentes a emissão da nota
pub struct Emissao {
    pub horario: DateTime<Utc>,
    pub tipo: TipoEmissao,
    pub finalidade: FinalidadeEmissao,
    pub processo: TipoProcessoEmissao,
    pub versao_processo: String,
}

/// Dados referentes a operação da nota
pub struct Operacao {
    pub horario: Option<DateTime<Utc>>,
    pub tipo: TipoOperacao,
    pub destino: DestinoOperacao,
    pub natureza: String,
    pub consumidor: TipoConsumidor,
    pub presenca: TipoPresencaComprador,
    pub intermediador: Option<TipoIntermediador>
}

impl Identificacao {

    /// Parse da seção <ide>
    pub(crate) fn parse(xml: Dom) -> Result<Self, String> {

        let mut t_ide = Dom::new(DomType::Tag);
        t_ide.set_tag(Tag::new("ide"));

        let ide = parsercher::search_dom(&xml, &t_ide)
            .ok_or("Tag <ide> não encontrada")?;

        let codigo_uf = parsercher::search_text_from_tag_children(&ide, &Tag::new("cUF"))
            .ok_or("Tag <cUF> não encontrada na <ide>")?[0]
            .parse::<u8>()
            .map_err(|e| e.to_string())?;

        let codigo_chave = parsercher::search_text_from_tag_children(&ide, &Tag::new("cNF"))
            .ok_or("Tag <cNF> não encontrada na <ide>")?[0]
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        let serie = parsercher::search_text_from_tag_children(&ide, &Tag::new("serie"))
            .ok_or("Tag <serie> não encontrada na <ide>")?[0]
            .parse::<u16>()
            .map_err(|e| e.to_string())?;

        let numero = parsercher::search_text_from_tag_children(&ide, &Tag::new("nNF"))
            .ok_or("Tag <nNF> não encontrada na <ide>")?[0]
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        let modelo = parsercher::search_text_from_tag_children(&ide, &Tag::new("mod"))
            .ok_or("Tag <mod> não encontrada na <ide>")?[0]
            .parse::<ModeloDocumentoFiscal>()?;

        let emissao = {

            let horario = parsercher::search_text_from_tag_children(&ide, &Tag::new("dhEmi"))
                .ok_or("Tag <dhEmi> não encontrada na <ide>")?[0]
                .parse::<DateTime<Utc>>()
                .map_err(|e| e.to_string())?;

            let tipo = parsercher::search_text_from_tag_children(&ide, &Tag::new("tpEmis"))
                .ok_or("Tag <tpEmis> não encontrada na <ide>")?[0]
                .parse::<TipoEmissao>()?;

            let finalidade = parsercher::search_text_from_tag_children(&ide, &Tag::new("finNFe"))
                .ok_or("Tag <finNfe> não encontrada na <ide>")?[0]
                .parse::<FinalidadeEmissao>()?;

            let processo = parsercher::search_text_from_tag_children(&ide, &Tag::new("procEmi"))
                .ok_or("Tag <procEmi> não encontrada na <ide>")?[0]
                .parse::<TipoProcessoEmissao>()?;

            let versao_processo = parsercher::search_text_from_tag_children(&ide, &Tag::new("verProc"))
                .ok_or("Tag <verProc> não encontrada na <ide>")?[0].to_string();

            Emissao {
                horario,
                tipo,
                finalidade,
                processo,
                versao_processo
            }
        };

        let operacao = {
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
                if let Some(dt) = parsercher::search_text_from_tag_children(&ide, &Tag::new("dhSaiEnt")) {
                    Some(dt[0].parse::<DateTime<Utc>>()
                         .map_err(|e| e.to_string())?)
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
                if let Some(dt) = parsercher::search_text_from_tag_children(&ide, &Tag::new("indIntermed")) {
                    Some(dt[0].parse::<TipoIntermediador>()?)
                } else {
                    None
                }
            };

            Operacao {
                natureza,
                tipo,
                destino,
                horario,
                consumidor,
                presenca,
                intermediador
            }
        };

        let codigo_municipio = parsercher::search_text_from_tag_children(&ide, &Tag::new("cMunFG"))
            .ok_or("Tag <cMunFG> não encontrada na <ide>")?[0]
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        let formato_danfe = parsercher::search_text_from_tag_children(&ide, &Tag::new("tpImp"))
            .ok_or("Tag <tpImp> não encontrada na <ide>")?[0]
            .parse::<FormatoImpressaoDanfe>()?;

        let ambiente = parsercher::search_text_from_tag_children(&ide, &Tag::new("tpAmb"))
            .ok_or("Tag <tpAmb> não encontrada na <ide>")?[0]
            .parse::<TipoAmbiente>()?;

        Ok(Identificacao {
            codigo_uf,
            codigo_chave,
            serie,
            numero,
            modelo,
            emissao,
            operacao,
            codigo_municipio,
            formato_danfe,
            ambiente
        })
    }
}

impl FromStr for ModeloDocumentoFiscal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "65" => ModeloDocumentoFiscal::Nfce,
            _ => ModeloDocumentoFiscal::Nfe // 55
        })
    }
}

impl FromStr for TipoOperacao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => TipoOperacao::Saida,
            _ => TipoOperacao::Entrada // 0
        })
    }
}

impl FromStr for DestinoOperacao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "3" => DestinoOperacao::ComExterior,
            "2" => DestinoOperacao::Interestadual,
            _ => DestinoOperacao::Interna // 1
        })
    }
}

impl FromStr for TipoEmissao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "2" => TipoEmissao::ContigenciaFsIa,
            "3" => TipoEmissao::ContingenciaScan,
            "4" => TipoEmissao::ContigenciaEpec,
            "5" => TipoEmissao::ContigenciaFsDa,
            "6" => TipoEmissao::ContigenciaSvcAn,
            "7" => TipoEmissao::ContigenciaSvcRs,
            "9" => TipoEmissao::ContigenciaOfflineNfce,
            _ => TipoEmissao::Normal // 1
        })
    }
}

impl FromStr for FormatoImpressaoDanfe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "5" => FormatoImpressaoDanfe::NfceMensagemEletronica,
            "4" => FormatoImpressaoDanfe::Nfce,
            "3" => FormatoImpressaoDanfe::Simplificado,
            "2" => FormatoImpressaoDanfe::NormalPaisagem,
            "1" => FormatoImpressaoDanfe::NormalRetrato,
            _ => FormatoImpressaoDanfe::SemGeracao // 0
        })
    }
}

impl FromStr for TipoAmbiente {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => TipoAmbiente::Producao,
            _ => TipoAmbiente::Homologacao // 2
        })
    }
}

impl FromStr for FinalidadeEmissao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "4" => FinalidadeEmissao::Devolucao,
            "3" => FinalidadeEmissao::Ajuste,
            "2" => FinalidadeEmissao::Complementar,
            _ => FinalidadeEmissao::Normal // 1
        })
    }
}

impl FromStr for TipoConsumidor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "2" => TipoConsumidor::Final,
            _ => TipoConsumidor::Normal // 1
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
            _ => TipoPresencaComprador::NaoSeAplica // 0
        })
    }
}

impl FromStr for TipoIntermediador {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => TipoIntermediador::EmSiteDeTerceiros,
            _ => TipoIntermediador::SemIntermediador // 0
        })
    }
}

impl FromStr for TipoProcessoEmissao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "3" => TipoProcessoEmissao::ViaAplicativoDoFisco,
            "2" => TipoProcessoEmissao::AvulsaPeloContribuinte,
            "1" => TipoProcessoEmissao::AvulsaPeloFisco,
            _ => TipoProcessoEmissao::ViaAplicativoDoContribuinte // 0
        })
    }
}
