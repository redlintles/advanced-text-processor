use std::borrow::Cow;

use crate::context::execution_context::{ GlobalExecutionContext };
use crate::utils::errors::AtpError;

use crate::utils::params::AtpParamTypes;

pub mod instructions;
pub mod transforms;

/// InstructionMethods
///
/// Basic Contract which every token should implement
pub trait InstructionMethods: InstructionMethodsClone + Send + Sync {
    /// to_atp_line
    ///
    /// Converts the token to an ATP line to be written in an .atp file
    fn to_atp_line(&self) -> Cow<'static, str>;
    /// transform
    ///
    /// Responsible for applying the respective token transformation to `input`
    fn transform(
        &self,
        input: &str,
        context: &mut GlobalExecutionContext
    ) -> Result<String, AtpError>;

    /// get_string_repr
    ///
    /// Converts the token to a string representation without parameters, to be used in the mappings
    fn get_string_repr(&self) -> &'static str;

    fn from_params(&mut self, params: &Vec<AtpParamTypes>) -> Result<(), AtpError>;

    fn get_params(&self) -> &Vec<AtpParamTypes>;
    /// BytecodeMethods
    #[cfg(feature = "bytecode")]
    fn to_bytecode(&self) -> Vec<u8>;

    #[cfg(feature = "bytecode")]
    fn get_opcode(&self) -> u32;
}

pub trait InstructionMethodsClone {
    fn clone_box(&self) -> Box<dyn InstructionMethods>;
}

impl<T> InstructionMethodsClone for T where T: 'static + InstructionMethods + Clone {
    fn clone_box(&self) -> Box<dyn InstructionMethods> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn InstructionMethods> {
    fn clone(&self) -> Box<dyn InstructionMethods> {
        self.clone_box()
    }
}
