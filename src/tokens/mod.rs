use std::borrow::Cow;

use crate::utils::errors::AtpError;
#[cfg(feature = "bytecode")]
use crate::utils::params::AtpParamTypes;

pub mod instructions;
pub mod transforms;

/// TokenMethods
///
/// Basic Contract which every token should implement

pub trait TokenMethods: TokenMethodsClone + Send + Sync {
    /// to_atp_line
    ///
    /// Converts the token to an ATP line to be written in an .atp file
    fn to_atp_line(&self) -> Cow<'static, str>;
    /// transform
    ///
    /// Responsible for applying the respective token transformation to `input`
    fn transform(&self, input: &str) -> Result<String, AtpError>;

    /// get_string_repr
    ///
    /// Converts the token to a string representation without parameters, to be used in the mappings
    fn get_string_repr(&self) -> &'static str;

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError>;
    /// BytecodeMethods
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8>;

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32;
}

pub trait TokenMethodsClone {
    fn clone_box(&self) -> Box<dyn TokenMethods>;
}

impl<T> TokenMethodsClone for T
where
    T: 'static + TokenMethods + Clone,
{
    fn clone_box(&self) -> Box<dyn TokenMethods> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn TokenMethods> {
    fn clone(&self) -> Box<dyn TokenMethods> {
        self.clone_box()
    }
}
