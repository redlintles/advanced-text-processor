use crate::utils::errors::AtpError;

pub mod token_defs;

/// TokenMethods
///
/// Basic Contract which every token should implement
pub trait TokenMethods: TokenMethodsClone {
    /// to_atp_line
    ///
    /// Converts the token to an ATP line to be written in an .atp file
    fn to_atp_line(&self) -> String;
    /// parse
    ///
    /// Responsible for applying the respective token transformation to `input`
    fn parse(&self, input: &str) -> Result<String, AtpError>;

    /// get_string_repr
    ///
    /// Converts the token to a string representation without parameters, to be used in the mappings
    fn get_string_repr(&self) -> String;

    /// from_vec_params
    ///
    /// Fills the token object params based on a String vec send by the lexer
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError>;
}

pub trait TokenMethodsClone {
    fn clone_box(&self) -> Box<dyn TokenMethods>;
}

impl<T> TokenMethodsClone for T where T: 'static + TokenMethods + Clone {
    fn clone_box(&self) -> Box<dyn TokenMethods> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn TokenMethods> {
    fn clone(&self) -> Box<dyn TokenMethods> {
        self.clone_box()
    }
}
