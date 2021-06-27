//! nfe - Crate para acesso aos dados da Nota Fiscal Eletrônica

mod nfe;

pub use crate::{
    nfe::*
};

#[cfg(test)]
mod tests;
