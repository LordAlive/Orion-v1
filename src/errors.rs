#[derive(Debug)]
pub enum VMError{
    StackUnderflow,
    UnexpectedEof,
    Invalid_Opcode
}

pub type VMResult<T> = Result<T,VMError>; 