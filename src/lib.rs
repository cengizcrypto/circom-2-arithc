//! # Circom To Arithmetic Circuit
//!
//! This library provides the functionality to convert a Circom program into an arithmetic circuit.

pub mod circom;
pub mod circuit;
pub mod cli;
pub mod process;
pub mod program;
pub mod runtime;

pub use cli::{build_output, Args};
