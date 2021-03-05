use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum Opcode {
  Halt,
  Out,
  Noop
}

impl From<u16> for Opcode {
  fn from(code: u16) -> Self {
    match code {
      0 => Opcode::Halt,
      19 => Opcode::Out,
      21 => Opcode::Noop,
      _ => panic!("Don't know opcode {}", code)
    }
  }
}


struct State {
  instructions: Vec<u16>,
  ip: usize
}

impl State {
  fn build(instructions: Vec<u16>) -> State {
    State { instructions, ip: 0 }
  }

  fn opcode(&self) -> Opcode {
    Opcode::from(self.instructions[self.ip])
  }

  fn increment_ip(&mut self, count: usize) {
    self.ip += count;
  }

  fn read_args(&mut self, count: usize) -> &[u16] {
    if count == 0 { panic!("Can't read zero args"); }

    self.ip += 1;
    let args = &self.instructions[self.ip..self.ip+count];
    self.ip += count;

    args
  }
}

fn read_num(mut f: &File, mut buf: [u8; 2]) -> Option<u16> {
  if let Ok(_) = (&mut f).read_exact(&mut buf) {
      let instruction = ((buf[1] as u16) << 8) | (buf[0] as u16);
      Some(instruction)
  } else {
    None
  }
}

pub fn read_binary(filename: &str) -> Vec<u16> {
  let f = File::open(filename).expect("Failed to open .bin file");

  let mut instructions = Vec::new();
  let buffer = [0; 2];

  while let Some(instruction) = read_num(&f, buffer) {
      instructions.push(instruction);
  }

  return instructions;
}

pub fn run_loop(instructions: Vec<u16>) {
  let mut state = State::build(instructions);

  loop {
    match state.opcode() {
      Opcode::Halt => {
        break;
      }
      Opcode::Out => {
        let byte = state.read_args(1)[0] as u8;
        let char = byte as char;
        print!("{}", char);
      }
      Opcode::Noop => {
        state.increment_ip(1);
        "Do nothing";
      }
    }
  };

  println!("Execution complete.");
}