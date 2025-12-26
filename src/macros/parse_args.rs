#[macro_export]
macro_rules! parse_args {
    ($params:expr, $idx:expr, String, $msg:expr) => {
        {
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
