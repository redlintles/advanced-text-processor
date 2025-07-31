#[cfg(not(feature = "test_access"))]
mod utils;

#[cfg(not(feature = "test_access"))]
mod tokens;

#[cfg(feature = "test_access")]
pub mod tokens;
#[cfg(feature = "test_access")]
pub mod utils;

#[cfg(not(feature = "test_access"))]
mod text;

#[cfg(feature = "test_access")]
pub mod text;
// Public

pub mod builder;

// Bytecode
#[cfg(feature = "bytecode")]
pub mod bytecode;
