mod opcodes;
mod error;
mod vm;

use vm::{VM, VMMemory};
use opcodes::*;

fn main() {
  let mut mem = VMMemory(vec![0; 10]);
  mem[0] = 1;
  mem[1] = 2; // не работает
  mem[2] = OP_NOP;
  mem[3] = OP_NOP;
  mem[4] = OP_NOP;
  mem[5] = OP_NOP;
  mem[6] = OP_NOP;
  mem[7] = OP_NOP;
  mem[8] = OP_NOP;
  mem[9] = OP_NOP;

  let mut vm = VM::new(mem);
  let result = vm.run();

  

  match result {
    Ok(_) => (),
    Err(error) => println!("Error: \n\t{}", error.to_string())
  }

  println!("o1");
}
