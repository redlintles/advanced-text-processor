#[macro_export]
macro_rules! parse_args {
    ($params:expr, $idx:expr, String, $msg:expr) => {
        {
        use crate::utils::params::AtpParamTypes;
        match &$params[$idx] {
            AtpParamTypes::String(payload) => payload.clone(),
            _ => {
                return Err(AtpError::new(
                    AtpErrorCode::InvalidParameters($msg.into()),
                    "",
                    "",
                ));
            }
        }
        }
    };
    ($params:expr, $idx:expr, Usize, $msg:expr) => {
        {
        use crate::utils::params::AtpParamTypes;
        match &$params[$idx] {
            AtpParamTypes::Usize(payload) => payload.clone(),
            _ => {
                return Err(AtpError::new(
                    AtpErrorCode::InvalidParameters($msg.into()),
                    "",
                    "",
                ));
            }
        }
        }
    };
    ($params:expr, $idx:expr, Token, $msg:expr) => {
        {
        use crate::utils::params::AtpParamTypes;
        match &$params[$idx] {
            AtpParamTypes::Token(payload) => payload.clone(),
            _ => {
                return Err(AtpError::new(
                    AtpErrorCode::InvalidParameters($msg.into()),
                    "",
                    "",
                ));
            }
        }
        }
    };
}
