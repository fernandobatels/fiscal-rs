//! nfe - Crate para acesso aos dados da Nota Fiscal Eletrônica

mod ide;
mod emit;
mod dest;
mod nfe;

pub use crate::{
    nfe::*,
    emit::*,
    dest::*,
    ide::*
};

#[cfg(test)]
mod tests;
