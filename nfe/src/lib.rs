//! nfe - Crate para acesso aos dados da Nota Fiscal Eletrônica

mod ide;
mod nfe;

pub use crate::{
    nfe::*,
    ide::*
};

#[cfg(test)]
mod tests;
