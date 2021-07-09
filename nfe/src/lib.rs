//! nfe - Crate para acesso aos dados da Nota Fiscal Eletrônica

mod comum;
mod nfe;

pub mod base;

pub use crate::{
    nfe::*,
    comum::*
};

#[cfg(test)]
mod tests;
