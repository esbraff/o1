mod op;
mod error;
mod vm;

use vm::{VM, VMMemory};
use op::*;

fn main() {
  let mut mem = VMMemory(vec![0; 10]);
  mem[0] = 1;
  mem[1] = 2; // неработает
  mem[2] = NOP;
  mem[3] = NOP;
  mem[4] = NOP;
  mem[5] = NOP;
  mem[6] = NOP;
  mem[7] = NOP;
  mem[8] = NOP;
  mem[9] = NOP;

  let mut vm = VM::new(mem);
  let result = vm.run();

  match result {
    Ok(_) => (),
    Err(error) => println!("Error: \n\t{}", error.to_string())
  }

  println!("o1");
}
