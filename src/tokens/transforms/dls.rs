use std::borrow::Cow;

use crate::{
    tokens::TokenMethods,
    utils::{
        errors::{ AtpError, AtpErrorCode },
        transforms::string_to_usize,
        validations::{ check_index_against_input, check_vec_len },
    },
};

#[cfg(feature = "bytecode")]
use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

/// DLS - Delete Single
///
/// Delete's a single character specified by `index` in `input`
///
/// It will throw an `AtpError` if index does not exists in `input`
///
/// # Example
///
/// ```rust
/// use atp_project::tokens::{TokenMethods, transforms::dls::Dls};
///
/// let token = Dls::params(3);
///
/// assert_eq!(token.parse("banana"), Ok("banna".to_string()));
/// ```
#[derive(Clone, Copy, Default)]
pub struct Dls {
    pub index: usize,
}

impl Dls {
    pub fn params(index: usize) -> Self {
        Dls {
            index,
        }
    }
}

impl TokenMethods for Dls {
    fn get_string_repr(&self) -> &'static str {
        "dls"
    }
    fn to_atp_line(&self) -> Cow<'static, str> {
        format!("dls {};\n", self.index).into()
    }

    fn parse(&self, input: &str) -> Result<String, AtpError> {
        check_index_against_input(self.index, input)?;
        Ok(
            input
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if self.index == i {
                        return None;
                    } else {
                        return Some(c);
                    }
                })
                .collect()
        )
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        check_vec_len(&line, 2)?;
        if line[0] == "dls" {
            self.index = string_to_usize(&line[1])?;
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".into()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
}

#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Dls {
    fn get_opcode(&self) -> u8 {
        0x32
    }

    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), AtpError> {
        check_vec_len(&instruction.operands, 1)?;
        if instruction.op_code == self.get_opcode() {
            use crate::utils::transforms::string_to_usize;

            self.index = string_to_usize(&instruction.operands[0])?;
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }

    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: self.get_opcode(),
            operands: [self.index.to_string()].to_vec(),
        }
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod dls_tests {
    use crate::tokens::{ TokenMethods, transforms::dls::Dls };

    #[test]
    fn delete_single_tests() {
        let mut token = Dls::params(3);

        assert_eq!(token.parse("banana"), Ok("banna".to_string()), "It supports expected inputs");
        assert_eq!(
            token.to_atp_line(),
            "dls 3;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(token.get_string_repr(), "dls".to_string(), "get_string_repr works as expected");
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(token.from_vec_params(["dls".to_string(), (3).to_string()].to_vec()), Ok(_)),
            "It does not throws an error for valid vec_params"
        );

        assert_eq!(
            token.parse("banana"),
            Ok("banna".to_string()),
            "from_vec_params parses the argument list correctly"
        );
    }

    #[cfg(feature = "bytecode")]
    #[test]
    fn delete_single_bytecode_tests() {
        use crate::bytecode::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Dls::params(3);

        let instruction = BytecodeInstruction {
            op_code: 0x32,
            operands: [(3).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x32, "get_opcode does not disrepect ATP token mapping");

        assert_eq!(
            token.token_from_bytecode_instruction(instruction.clone()),
            Ok(()),
            "Parsing from bytecode to token works correctly!"
        );

        assert_eq!(
            token.token_to_bytecode_instruction(),
            instruction,
            "Conversion to bytecode instruction works perfectly!"
        );
    }
}
