#[cfg(not(feature = "test_access"))]
pub mod utils;

#[cfg(not(feature = "test_access"))]
mod tokens;

#[cfg(feature = "test_access")]
pub mod token_data;

#[cfg(not(feature = "test_access"))]
mod text;

#[cfg(feature = "test_access")]
pub mod text_parser;
// Public

pub mod builder;

// Bytecode
#[cfg(feature = "bytecode")]
pub mod bytecode_parser;
