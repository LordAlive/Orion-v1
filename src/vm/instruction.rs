
#[derive(Debug,Clone)]
pub enum Instruction{
    PUSH(usize),
    // POP,
    MSTORE,
    MLOAD,
    SSTORE,
    SLOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HALT,
    DESTRUCT,
    PRINT
}
