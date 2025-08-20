pub mod stack;
pub mod instruction;
use crate::errors::{VMError,VMResult};
use instruction::Instruction;
use stack::Stack; 

pub struct VM{
    pc:usize,
    stack:Stack,
    program: Vec<Instruction>
}

impl VM{
    pub fn new(program:Vec<Instruction>)->Self{
        Self{pc:0,stack:Stack::default(),program}
    }
    pub fn run(self:&mut Self)->VMResult<()>{
        while self.pc < self.program.len(){
            let inst = self.program.get(self.pc).ok_or(VMError::UnexpectedEof)?.clone(); 
            self.pc +=1; 
            self.step(inst); 
        }
        Ok(())
    }
    pub fn step(&mut self, instruction: Instruction)->VMResult<()>{
        match instruction {
            Instruction::ADD =>{
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a+b); 

            }
            Instruction::DIV =>{
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a/b); 
            }
            Instruction::DESTRUCT =>{}
            Instruction::HALT =>{
                self.pc = self.program.len(); 
            }
            Instruction::MLOAD =>{}
            Instruction::MSTORE =>{}
            Instruction::MUL => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a*b); 
            }
            Instruction::PUSH(v) => {
                self.stack.push(v.try_into().unwrap());
            }
            Instruction::SLOAD => {}
            Instruction::SSTORE => {}
            Instruction::SUB => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a-b); 
            }
            Instruction::PRINT =>{
                let v = self.stack.pop()?; 
                println!("printed val is {}",v); 
            }
        }
        Ok(())
    }
}