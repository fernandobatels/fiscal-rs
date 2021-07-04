//! nfe - Crate para acesso aos dados da Nota Fiscal Eletrônica

mod ide;
mod emit;
mod nfe;

pub use crate::{
    nfe::*,
    emit::*,
    ide::*
};

#[cfg(test)]
mod tests;
