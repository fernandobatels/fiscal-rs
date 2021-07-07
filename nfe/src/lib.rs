//! nfe - Crate para acesso aos dados da Nota Fiscal Eletrônica

mod padrao;
mod nfe;

pub mod base;

pub use crate::{
    nfe::*,
    padrao::*
};

#[cfg(test)]
mod tests;
