mod errors;
mod vm; 
use errors::VMResult; 
use vm::instruction::Instruction as I; 
use vm::VM; 
fn main()->VMResult<()>{
    let inst = vec![I::PUSH((10)),I::PUSH((20)),I::ADD,I::PRINT]; 
    let result = VM::new(inst).run(); 
    result
}