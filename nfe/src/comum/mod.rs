//! Tipos e módulos comuns a toda crate

pub(crate) mod emissao;
pub(crate) mod operacao;
pub(crate) mod ide;
pub(crate) mod dest;
pub(crate) mod emit;
pub(crate) mod versao;
pub(crate) mod endereco;

pub use emissao::*;
pub use operacao::*;
pub use ide::*;
pub use dest::*;
pub use emit::*;
pub use versao::*;
pub use endereco::*;
