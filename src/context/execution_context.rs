use std::collections::HashMap;

use crate::{ tokens::TokenMethods, utils::errors::AtpError };

pub enum VarValues {
    String(String),
    Usize(usize),
}

pub enum ToClean {
    Block(String),
    Var(String),
}
// First thought of a simple hashmap, but it wouldn't suffice my needs
pub struct VarEntry {
    value: VarValues,
    mutable: bool,
}
// This Object will be re-created every time the program starts.
// Some tokens could access this object for additional data
pub struct GlobalExecutionContext {
    variables: HashMap<String, VarEntry>,
    blocks: HashMap<String, Vec<Box<dyn TokenMethods>>>,
}

// Variable Concept

// val {name} = {value}; for immutable variables
// var {name} = {value}; for mutable variables
// then the user could reference the variable by ${name} syntax
// And alter it throught mut {name} = {new_value}; instruction if it's mutable.

// Block concept

// blk {name} assoc {instruction};
// if {name} block doesn't exist yet, it's created
// The instruction will be parsed to a Box<dyn TokenMethods> and then,
// added to the {name} block in the Context's blocks hashmap.
// If the user wish to add multiple instructions to a single block, it should do one per `blk assoc` line.
// Once the user is done with composing a block
// cblk {name}; will execute all instructions stored in the {name} block;

pub trait GlobalContextMethods {
    fn add_to_block(
        &mut self,
        block_id: &str,
        token: Box<dyn TokenMethods>
    ) -> Result<(), AtpError>;
    fn get_block(&self, block_id: &str) -> Result<&Vec<Box<dyn TokenMethods>>, AtpError>;

    fn add_var(&mut self, var_entry: VarEntry) -> Result<(), AtpError>;
    fn get_var(&self, var_id: &str) -> Result<&VarEntry, AtpError>;
    fn get_mut_var(&mut self, var_id: &str) -> Result<&mut VarEntry, AtpError>;

    // It would require a more complex implementation. but would help optimizing atp in the future. This will remove data that will no longer be used from the context.
    fn clean_context(&mut self) -> ();
}
