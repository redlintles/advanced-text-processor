use std::collections::HashMap;
use crate::{ tokens::InstructionMethods, utils::errors::{ AtpError, AtpErrorCode } };
#[derive(Clone)]
pub enum VarValues {
    String(String),
    Usize(usize),
    Token(Box<dyn InstructionMethods>),
}

pub enum ToClean {
    Block(String),
    Var(String),
}

// First thought of a simple hashmap, but it wouldn't suffice my needs
#[allow(dead_code)]
pub struct VarEntry {
    pub value: VarValues,
    pub mutable: bool,
}
// This Object will be re-created every time the program starts.
// Some tokens could access this object for additional data
pub struct GlobalExecutionContext {
    variables: HashMap<String, VarEntry>,
    blocks: HashMap<String, Vec<Box<dyn InstructionMethods>>>,
}

// Variable Concept

// val {name} = {value}; for immutable variables
// var {name} = {value}; for mutable variables
// then the user could reference the variable by ${name} syntax
// And alter it throught mut {name} = {new_value}; instruction if it's mutable.

// Block concept

// blk {name} assoc {instruction};
// if {name} block doesn't exist yet, it's created
// The instruction will be parsed to a Box<dyn InstructionMethods> and then,
// added to the {name} block in the Context's blocks hashmap.
// If the user wish to add multiple instructions to a single block, it should do one per `blk assoc` line.
// Once the user is done with composing a block
// cblk {name}; will execute all instructions stored in the {name} block;

pub trait GlobalContextMethods {
    fn add_to_block(
        &mut self,
        block_id: &str,
        token: Box<dyn InstructionMethods>
    ) -> Result<(), AtpError>;
    fn get_formatted_block_items(&mut self, block_id: &str) -> Result<String, AtpError>;

    fn add_var(&mut self, id: &str, var_entry: VarEntry) -> Result<(), AtpError>;
    fn get_var(&self, var_id: &str) -> Result<&VarEntry, AtpError>;
    fn get_mut_var(&mut self, var_id: &str) -> Result<&mut VarEntry, AtpError>;

    // It would require a more complex implementation. but would help optimizing atp in the future. This will remove data that will no longer be used from the context.
    fn clean_context(&mut self) -> () {}
    fn take_block(&mut self, block_id: &str) -> Result<Vec<Box<dyn InstructionMethods>>, AtpError>;
    fn put_block(&mut self, block_id: &str, block: Vec<Box<dyn InstructionMethods>>);
}

impl GlobalExecutionContext {
    pub fn new() -> Self {
        GlobalExecutionContext { variables: HashMap::new(), blocks: HashMap::new() }
    }
}

impl GlobalContextMethods for GlobalExecutionContext {
    fn add_to_block(
        &mut self,
        block_id: &str,
        token: Box<dyn InstructionMethods>
    ) -> Result<(), AtpError> {
        match self.blocks.get_mut(block_id) {
            Some(tokens) => {
                tokens.push(token);
            }
            None => {
                let mut block_vec = Vec::new();
                block_vec.push(token);

                self.blocks.insert(block_id.to_string(), block_vec);
            }
        }

        Ok(())
    }

    fn take_block(&mut self, block_id: &str) -> Result<Vec<Box<dyn InstructionMethods>>, AtpError> {
        self.blocks
            .remove(block_id)
            .ok_or_else(|| {
                AtpError::new(
                    AtpErrorCode::BlockNotFound("Block not found".into()),
                    "context.take_block",
                    block_id.to_string()
                )
            })
    }

    fn put_block(&mut self, block_id: &str, block: Vec<Box<dyn InstructionMethods>>) {
        self.blocks.insert(block_id.to_string(), block);
    }

    fn get_formatted_block_items(&mut self, block_id: &str) -> Result<String, AtpError> {
        use colored::Colorize;

        let block_items = self.take_block(block_id)?;
        let mut result = String::new();

        let len = block_items.len();
        if len == 0 {
            result.push_str("\t\t\t\t(EMPTY BLOCK)\n");
            return Ok(result);
        }

        for (i, token) in block_items.iter().enumerate() {
            let is_last = i + 1 == len;

            let prefix = if is_last {
                if len == 1 {
                    "(BLOCK CREATED): ".green()
                } else {
                    "(BLOCK ALREADY EXISTS) ADDING: ".green()
                }
            } else {
                // sem prefixo para itens antigos
                "".normal()
            };

            result.push_str(&format!("\t\t\t\t{}{}\n", prefix, token.to_atp_line().yellow()));
        }

        self.put_block(block_id, block_items);

        Ok(result)
    }

    fn add_var(&mut self, id: &str, var_entry: VarEntry) -> Result<(), AtpError> {
        self.variables.insert(id.to_string(), var_entry);
        Ok(())
    }

    fn get_var(&self, var_id: &str) -> Result<&VarEntry, AtpError> {
        Ok(
            self.variables
                .get(var_id)
                .ok_or_else(||
                    AtpError::new(
                        AtpErrorCode::VariableNotFound("Variable not found".into()),
                        "get_var",
                        var_id.to_string()
                    )
                )?
        )
    }

    fn get_mut_var(&mut self, var_id: &str) -> Result<&mut VarEntry, AtpError> {
        let v = self.variables
            .get_mut(var_id)
            .ok_or_else(||
                AtpError::new(
                    AtpErrorCode::VariableNotFound("Variable not found".into()),
                    "get_var",
                    var_id.to_string()
                )
            )?;
        if v.mutable {
            Ok(v)
        } else {
            Err(
                AtpError::new(
                    AtpErrorCode::NonMutableVariableError("Variable is not mutable".into()),
                    "get_mut_var",
                    var_id.to_string()
                )
            )
        }
    }
}
