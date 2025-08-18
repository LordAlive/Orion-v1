 
// use crate::errors::VMResult; 
pub struct Stack(Vec<i64>); 


impl Stack {
    pub fn push(&mut self,v:i64 ){
        self.0.push(v);
    }
    pub fn pop(&mut self){
        self.0.pop(); 
    }
    pub fn len(&self)->usize{
        self.0.len() 
    }
}