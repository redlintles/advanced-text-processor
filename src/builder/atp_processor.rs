use std::collections::HashMap;
use std::path::Path;

use uuid::Uuid;

#[cfg(any(feature = "debug", feature = "bytecode_debug"))]
use colored::*;

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{
    reader::read_bytecode_from_file,
    writer::write_bytecode_to_file,
    BytecodeTokenMethods,
};
use crate::token_data::{ TokenMethods };

use crate::text_parser::parser::parse_token;
use crate::text_parser::reader::read_from_file;
use crate::text_parser::writer::write_to_file;

use crate::utils::errors::{ token_array_not_found, ErrorManager };
#[cfg(feature = "bytecode")]
use crate::utils::transforms::{ bytecode_token_vec_to_token_vec, token_vec_to_bytecode_token_vec };
#[derive(Default)]
pub struct AtpProcessor {
    transforms: HashMap<String, Vec<Box<dyn TokenMethods>>>,
    errors: ErrorManager,
}

pub trait AtpProcessorMethods {
    fn write_to_text_file(&self, id: &str, path: &Path) -> Result<(), String>;
    fn read_from_text_file(&mut self, path: &Path) -> Result<String, String>;
    fn add_transform(&mut self, tokens: Vec<Box<dyn TokenMethods>>) -> String;
    fn process_all(&self, id: &str, input: &str) -> Result<String, String>;
    fn process_single(&self, token: Box<dyn TokenMethods>, input: &str) -> String;
}
#[cfg(feature = "bytecode")]
pub trait AtpProcessorBytecodeMethods: AtpProcessorMethods {
    fn write_to_bytecode_file(&self, id: &str, path: &Path) -> Result<(), String>;
    fn read_from_bytecode_file(&mut self, path: &Path) -> Result<(), String>;
}
#[cfg(feature = "debug")]
pub trait AtpProcessorDebugMethods: AtpProcessorMethods {
    fn process_all_with_debug(&self, id: &str, input: &str) -> Result<String, String>;
    fn process_single_with_debug(&self, token: Box<dyn TokenMethods>, input: &str) -> String;
}

#[cfg(feature = "bytecode_debug")]
pub trait AtpProcessorBytecodeDebugMethods: AtpProcessorBytecodeMethods {
    fn process_all_bytecode_with_debug(&self, id: &str, input: &str) -> Result<String, String>;
    fn process_single_bytecode_with_debug(
        &self,
        token: Box<dyn BytecodeTokenMethods>,
        input: &str
    ) -> String;
}

impl AtpProcessor {
    pub fn new() -> Self {
        AtpProcessor { transforms: HashMap::new(), errors: ErrorManager::default() }
    }
}

impl AtpProcessorMethods for AtpProcessor {
    fn write_to_text_file(&self, id: &str, path: &Path) -> Result<(), String> {
        let tokens = self.transforms.get(id).ok_or_else(token_array_not_found(id))?;

        write_to_file(Path::new(path), tokens)
    }

    fn read_from_text_file(&mut self, path: &Path) -> Result<String, String> {
        let tokens = read_from_file(Path::new(path))?;

        let identifier = Uuid::new_v4();

        self.transforms.insert(identifier.to_string(), tokens);

        Ok(identifier.to_string())
    }

    fn process_all(&self, id: &str, input: &str) -> Result<String, String> {
        let mut result = String::from(input);

        let tokens = self.transforms.get(id).ok_or_else(token_array_not_found(id))?;

        for token in tokens.iter() {
            result = parse_token(token.as_ref(), result.as_str());
        }

        Ok(result.to_string())
    }

    fn add_transform(&mut self, tokens: Vec<Box<dyn TokenMethods>>) -> String {
        let identifier = Uuid::new_v4();
        self.transforms.insert(identifier.to_string(), tokens.clone());

        identifier.to_string()
    }

    fn process_single(&self, token: Box<dyn TokenMethods>, input: &str) -> String {
        token.parse(input)
    }
}

#[cfg(feature = "bytecode")]
impl AtpProcessorBytecodeMethods for AtpProcessor {
    fn write_to_bytecode_file(&self, id: &str, path: &Path) -> Result<(), String> {
        let tokens = self.transforms.get(id).ok_or_else(token_array_not_found(id))?;

        write_bytecode_to_file(path, token_vec_to_bytecode_token_vec(tokens)?)
    }
    fn read_from_bytecode_file(&mut self, path: &Path) -> Result<(), String> {
        let tokens = read_bytecode_from_file(path)?;

        self.add_transform(bytecode_token_vec_to_token_vec(&tokens)?);

        Ok(())
    }
}

#[cfg(feature = "debug")]
impl AtpProcessorDebugMethods for AtpProcessor {
    fn process_all_with_debug(&self, id: &str, input: &str) -> Result<String, String> {
        let mut result = String::from(input);

        let dashes = 10;

        let tokens = self.transforms.get(id).ok_or_else(token_array_not_found(id))?;

        println!("PROCESSING STEP BY STEP:\n{}\n", "-".repeat(dashes));

        for (counter, token) in (0_i64..).zip(tokens.iter()) {
            let temp = parse_token(token.as_ref(), result.as_str());

            println!(
                "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
                counter.to_string().blue(),
                (counter + 1).to_string().blue(),
                token.token_to_atp_line().yellow(),
                result.red(),
                temp.green()
            );

            if (counter as usize) < tokens.len() {
                println!("{}\n", "-".repeat(dashes));
            }

            result = temp;
        }

        Ok(result.to_string())
    }

    fn process_single_with_debug(&self, token: Box<dyn TokenMethods>, input: &str) -> String {
        let output = token.parse(input);
        println!(
            "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
            (0).to_string().blue(),
            (1).to_string().blue(),
            token.token_to_atp_line().yellow(),
            input.red(),
            output.green()
        );

        output
    }
}
#[cfg(feature = "bytecode_debug")]
impl AtpProcessorBytecodeDebugMethods for AtpProcessor {
    fn process_all_bytecode_with_debug(&self, id: &str, input: &str) -> Result<String, String> {
        let mut result = String::from(input);

        let dashes = 10;

        let tokens = token_vec_to_bytecode_token_vec(
            self.transforms.get(id).ok_or_else(token_array_not_found(id))?
        )?;

        println!("PROCESSING STEP BY STEP:\n{}\n", "-".repeat(dashes));

        for (counter, token) in (0_i64..).zip(tokens.iter()) {
            let temp = parse_token(token.as_ref(), result.as_str());
            println!(
                "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
                counter.to_string().blue(),
                (counter + 1).to_string().blue(),
                token.token_to_atp_line().yellow(),
                result.red(),
                temp.green()
            );

            if (counter as usize) < tokens.len() {
                println!("{}\n", "-".repeat(dashes));
            }

            result = temp;
        }

        Ok(result.to_string())
    }

    fn process_single_bytecode_with_debug(
        &self,
        token: Box<dyn BytecodeTokenMethods>,
        input: &str
    ) -> String {
        let output = token.parse(input);
        println!(
            "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
            (0).to_string().blue(),
            (1).to_string().blue(),
            token.token_to_atp_line().yellow(),
            input.red(),
            output.green()
        );

        output
    }
}
