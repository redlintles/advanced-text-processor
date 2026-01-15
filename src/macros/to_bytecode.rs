#[macro_export]
macro_rules! to_bytecode {
    ($opcode:expr, [$($param:expr),* $(,)?]) => {
        {
        use crate::context::execution_context::GlobalExecutionContext;
        // Coleta os params pra contar e iterar
        let params_vec: Vec<crate::utils::params::AtpParamTypes> = vec![$($param),*];

        let opcode_u32: u32 = $opcode;
        let param_count_u8: u8 = params_vec
            .len()
            .try_into()
            .expect("Param count exceeds u8::MAX");

        // Body = [opcode u32][param_count u8][params...]
        let mut body: Vec<u8> = Vec::new();
        body.extend_from_slice(&opcode_u32.to_be_bytes());
        body.push(param_count_u8);

        let mut ctx = GlobalExecutionContext::new();

        for p in &params_vec {
            p.write_as_instruction_param(&mut body, &mut ctx);
        }

        // Instruction Total Size = bytes do body (4 + 1 + params...)
        let instruction_total_size_u64: u64 = body.len() as u64;

        // Final = [total_size u64] + body
        let mut out: Vec<u8> = Vec::with_capacity(8 + body.len());
        out.extend_from_slice(&instruction_total_size_u64.to_be_bytes());
        out.extend_from_slice(&body);

        out
        }
    };
}
