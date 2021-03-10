use crate::{exec::Instruction, opcode::Opcode};
use super::util;

fn process_arg(arg: u16) -> String {
  if let Some(ascii) = util::to_ascii(arg) {
    format!("({}){}", ascii, arg)
  } else {
    let arg = util::register_pretty(&arg);
    arg
  }
}

fn print_instruction(ip: usize, raw: u16, opcode: &Opcode, args: Vec<u16>) {
  let args = args.iter().map(|n| process_arg(*n));
  let args: Vec<_> = args.collect();
  let args = args.join(", ");

  let opcode_str = match opcode {
    Opcode::Unknown => raw.to_string(),
    _ => opcode.to_string()
  };

  println!("{:5}: {} {}", ip, opcode_str, args);

  match opcode {
    Opcode::Ret => println!(""),
    _ => ()
  }
}

pub fn disassemble_instructions(instructions: Vec<u16>) {
  let mut ip = 0;

  while ip < instructions.len() {
    let opcode = Opcode::from(instructions[ip]);
    ip += 1;

    let args = &instructions[ip..ip + opcode.arg_count()];

    print_instruction(ip - 1, instructions[ip - 1], &opcode, args.to_vec());

    ip += opcode.arg_count();
  }
}