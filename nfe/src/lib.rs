//! nfe - Crate para acesso aos dados da NF-e
//!
//! Esta crate fornece uma interface fortemente tipada
//! para a NF-e. Sendo assim, muitos campos do XML são
//! disponibilizados em mais de uma struct.
//!
//! NFe x NFCe:
//!
//! No momento, apenas o modelo 55(NFe) é fornecido. Para manipular
//! o modelo 65(NFCe), você pode usar a base da NF-e e, se for necessário,
//! criar uma interface em cima disso.

pub mod base;
pub mod modelos;

pub use crate::modelos::nfe::*;

#[cfg(test)]
mod tests;
