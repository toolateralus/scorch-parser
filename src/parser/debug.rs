use crate::lexer::Token;

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dbgmsg {
    ($msg:expr) => {
        format!(" [{}:{}] {}", file!(), line!(), $msg)
    };
}
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dbgmsg {
    ($msg:expr) => {
        format!("{}", $msg)
    };
}

#[derive(Debug, Clone)]
pub enum ErrType {
    UnexpectedToken,
    UnexpectedEof,
}
#[derive(Debug, Clone)]
pub struct PrsErr {
    pub message: String,
    pub token: Token,
    pub type_: ErrType,
    pub index: usize,
    pub inner_err: Option<Box<PrsErr>>,
}
