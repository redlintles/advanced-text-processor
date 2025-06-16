use crate::utils::errors::AtpError;

pub mod token_defs;

pub trait TokenMethods: TokenMethodsClone {
    fn token_to_atp_line(&self) -> String;
    fn parse(&self, input: &str) -> Result<String, AtpError>;
    fn get_string_repr(&self) -> String;
    fn token_from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError>;
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
