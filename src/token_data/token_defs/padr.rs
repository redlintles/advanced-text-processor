use crate::{
    token_data::TokenMethods,
    utils::{ errors::{ AtpError, AtpErrorCode }, transforms::extend_string },
};

#[cfg(feature = "bytecode")]
use crate::bytecode_parser::{ BytecodeTokenMethods, BytecodeInstruction };
/// PADR - Pad Right
///
/// Repeats `text` characters until `max_len` is reached, and then insert the result at the end of `input`
///
/// See Also:
///
/// - [`Padr` - Pad Left](crate::token_data::token_defs::padr)
///
/// # Example:
///
/// ```rust
/// use atp_project::token_data::{TokenMethods, token_defs::padr::Padr};
///
/// let token = Padr::params("xy", 7);
///
/// assert_eq!(token.parse("banana"), Ok("bananax".to_string()));
/// ```
#[derive(Clone, Default)]
pub struct Padr {
    pub text: String,
    pub max_len: usize,
}

impl Padr {
    pub fn params(text: &str, max_len: usize) -> Self {
        Padr {
            text: text.to_string(),
            max_len,
        }
    }
}

impl TokenMethods for Padr {
    fn get_string_repr(&self) -> String {
        "padr".to_string()
    }
    fn to_atp_line(&self) -> String {
        format!("padr {} {};\n", self.text, self.max_len)
    }
    fn parse(&self, input: &str) -> Result<String, AtpError> {
        let character_count = input.chars().count();

        if character_count >= self.max_len {
            return Ok(input.to_string());
        }
        let ml = self.max_len - character_count;
        let s = extend_string(&self.text, ml);

        Ok(format!("{}{}", input, s))
    }
    fn from_vec_params(&mut self, line: Vec<String>) -> Result<(), AtpError> {
        if line[0] == "padr" {
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::TokenNotFound("Invalid Parser for this token".to_string()),
                line[0].to_string(),
                line.join(" ")
            )
        )
    }
}
#[cfg(feature = "bytecode")]
impl BytecodeTokenMethods for Padr {
    fn get_opcode(&self) -> u8 {
        0x30
    }
    fn token_from_bytecode_instruction(
        &mut self,
        instruction: BytecodeInstruction
    ) -> Result<(), crate::utils::errors::AtpError> {
        if instruction.op_code == self.get_opcode() {
            use crate::utils::transforms::string_to_usize;

            self.text = instruction.operands[0].clone();
            self.max_len = string_to_usize(&instruction.operands[1])?;
            return Ok(());
        }

        Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".to_string()),
                instruction.op_code.to_string(),
                instruction.operands.join(" ")
            )
        )
    }
    fn token_to_bytecode_instruction(&self) -> BytecodeInstruction {
        BytecodeInstruction {
            op_code: self.get_opcode(),
            operands: [self.text.clone(), self.max_len.to_string()].to_vec(),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "test_access")]
mod padr_tests {
    use crate::token_data::{ TokenMethods, token_defs::padr::Padr };
    #[test]
    fn pad_right_tests() {
        let mut token = Padr::params("xy", 7);
        assert_eq!(token.parse("banana"), Ok("bananax".to_string()), "It supports expected inputs");

        assert_eq!(
            token.to_atp_line(),
            "padr xy 7;\n".to_string(),
            "conversion to atp_line works correctly"
        );
        assert_eq!(
            token.get_string_repr(),
            "padr".to_string(),
            "get_string_repr works as expected"
        );
        assert!(
            matches!(token.from_vec_params(["tks".to_string()].to_vec()), Err(_)),
            "It throws an error for invalid vec_params"
        );
        assert!(
            matches!(
                token.from_vec_params(
                    ["padr".to_string(), "xy".to_string(), (7).to_string()].to_vec()
                ),
                Ok(_)
            ),
            "It does not throws an error for valid vec_params"
        );
    }
    #[cfg(feature = "bytecode")]
    #[test]
    fn pad_right_bytecode_tests() {
        use crate::bytecode_parser::{ BytecodeInstruction, BytecodeTokenMethods };

        let mut token = Padr::default();

        let instruction = BytecodeInstruction {
            op_code: 0x30,
            operands: ["xy".to_string(), (7).to_string()].to_vec(),
        };

        assert_eq!(token.get_opcode(), 0x30, "get_opcode does not disrepect ATP token mapping");

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
