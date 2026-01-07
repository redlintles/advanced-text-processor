#![cfg(feature = "test_access")]

#[cfg(test)]
mod tests {
    use crate::tokens::{ InstructionMethods, transforms::tucs::Tucs };
    use crate::utils::errors::{ AtpError, AtpErrorCode };
    use crate::utils::params::AtpParamTypes;

    #[test]
    fn get_string_repr_is_tucs() {
        let t = Tucs::default();
        assert_eq!(t.get_string_repr(), "tucs");
    }

    #[test]
    fn to_atp_line_is_correct() {
        let t = Tucs::params(1);
        assert_eq!(t.to_atp_line().as_ref(), "tucs 1;\n");
    }

    #[test]
    fn transform_uppercases_single_char() {
        let t = Tucs::params(1);
        assert_eq!(t.transform("banana"), Ok("bAnana".to_string()));
    }

    #[test]
    fn from_params_accepts_one_usize() {
        let mut t = Tucs::default();
        let params = vec![AtpParamTypes::Usize(2)];

        assert_eq!(t.from_params(&params), Ok(()));
        assert_eq!(t.to_atp_line().as_ref(), "tucs 2;\n");
    }

    #[test]
    fn from_params_rejects_wrong_len() {
        let mut t = Tucs::default();
        let params: Vec<AtpParamTypes> = vec![];

        let got = t.from_params(&params);

        let expected = Err(
            AtpError::new(
                AtpErrorCode::BytecodeNotFound("Invalid Parser for this token".into()),
                "",
                ""
            )
        );

        assert_eq!(got, expected);
    }

    // ============================
    // Bytecode tests
    // ============================
    #[cfg(feature = "bytecode")]
    mod bytecode_tests {
        use super::*;

        #[test]
        fn get_opcode_is_0x14() {
            let t = Tucs::default();
            assert_eq!(t.get_opcode(), 0x14);
        }

        #[test]
        fn to_bytecode_has_opcode_and_one_param() {
            let t = Tucs::params(5);
            let bc = t.to_bytecode();

            assert!(bc.len() >= 13);

            let total_size = u64::from_be_bytes(bc[0..8].try_into().unwrap()) as usize;
            assert_eq!(total_size, bc.len() - 8);

            let opcode = u32::from_be_bytes(bc[8..12].try_into().unwrap());
            assert_eq!(opcode, 0x14);

            let param_count = bc[12] as usize;
            assert_eq!(param_count, 1);
        }
    }
}
