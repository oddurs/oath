//! Oath: a language where a definition is a promise and an implementation is
//! sworn against it.
//!
//! The binary is a thin wrapper; everything the language does lives here so that
//! it can be tested without a subprocess.

pub mod diagnostic;
pub mod syntax;
