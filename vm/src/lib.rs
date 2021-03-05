use std::io::prelude::*;
use std::{collections::HashMap, fs::File, mem};

#[derive(Debug)]
enum Opcode {
    Halt,
    Set,
    Out,
    Jmp,
    JmpIfTrue,
    JmpIfFalse,
    Noop,
    Add,
    Mult,
    Mod,
    Eq,
    Push,
    Pop,
    Gt,
    And,
    Or,
    Not,
    Call,
    RMem,
    WMem,
    Ret,
    In
}

impl From<u16> for Opcode {
    fn from(code: u16) -> Self {
        match code {
            0 => Opcode::Halt,
            1 => Opcode::Set,
            2 => Opcode::Push,
            3 => Opcode::Pop,
            4 => Opcode::Eq,
            5 => Opcode::Gt,
            6 => Opcode::Jmp,
            7 => Opcode::JmpIfTrue,
            8 => Opcode::JmpIfFalse,
            9 => Opcode::Add,
            10 => Opcode::Mult,
            11 => Opcode::Mod,
            12 => Opcode::And,
            13 => Opcode::Or,
            14 => Opcode::Not,
            15 => Opcode::RMem,
            16 => Opcode::WMem,
            17 => Opcode::Call,
            18 => Opcode::Ret,
            19 => Opcode::Out,
            20 => Opcode::In,
            21 => Opcode::Noop,
            _ => panic!("Don't know opcode {}", code),
        }
    }
}

struct State {
    instructions: HashMap<u16, u16>,
    registers: Vec<u16>,
    ip: usize,
    stack: Vec<u16>,
    text_buffer: Option<String>
}

impl State {
    fn build(instructions: Vec<u16>) -> State {
        let instructions = instructions
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i as u16, v))
            .collect();

        State {
            instructions,
            registers: vec![0, 0, 0, 0, 0, 0, 0, 0],
            ip: 0,
            stack: vec![],
            text_buffer: None
        }
    }

    fn opcode(&self) -> Opcode {
        Opcode::from(self.instructions[&(self.ip as u16)])
    }

    fn jump_by(&mut self, count: usize) {
        self.ip += count;
    }

    fn jump_to(&mut self, to: u16) {
        self.ip = to as usize;
    }

    fn set_register(&mut self, index: u16, value: u16) {
        let register_index = index - 32768;
        self.registers[register_index as usize] = value;
    }

    fn resolve_value(&self, value: u16) -> u16 {
        if value <= 32767 {
            value
        } else {
            let register_index = value - 32768;
            self.registers[register_index as usize]
        }
    }

    fn read_next(&mut self) -> u16 {
        self.read_many(1)[0]
    }

    fn read_many(&mut self, count: usize) -> Vec<u16> {
        if count == 0 {
            panic!("Can't read zero args");
        }

        let mut values = Vec::new();

        self.ip += 1;

        for i in self.ip..self.ip + count {
          values.push(self.instructions[&(i as u16)]);
        }

        self.ip += count;

        values
    }

    fn push(&mut self, value: u16) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<u16> {
        self.stack.pop()
    }

    fn read_mem(&self, address: u16) -> u16 {
        self.instructions[&address]
    }

    fn write_mem(&mut self, address: u16, value: u16) {
        self.instructions.insert(address, value);
    }

    fn is_buffering_string(&self) -> bool {
      self.text_buffer.is_some()
    }

    fn text_buffer_char(&mut self) ->  u16 {
      let c = self.text_buffer.as_mut();
      let c = c.unwrap();
      let c = c.pop().unwrap();
      let c = c as u8;

      if c == '\n' as u8 {
        self.text_buffer = None;
      }

      return c as u16;
    }

    fn start_buffering_string(&mut self, s: String) {
      self.text_buffer = Some(s.chars().rev().collect());
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
            Opcode::Add => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                let result = ((b as usize + c as usize) % 32768) as u16;
                state.set_register(a, result);
            }
            Opcode::Mult => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                let result = ((b as usize * c as usize) % 32768) as u16;
                state.set_register(a, result);
            }
            Opcode::Mod => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                let result = b % c;
                state.set_register(a, result);
            }
            Opcode::Push => {
                let value = state.read_next();
                let a = state.resolve_value(value);
                state.push(a);
            }
            Opcode::Pop => {
                let value = state.pop().unwrap();
                let a = state.read_next();
                state.set_register(a, value);
            }
            Opcode::Gt => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                if b > c {
                    state.set_register(a, 1);
                } else {
                    state.set_register(a, 0);
                }
            }
            Opcode::Eq => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                if b == c {
                    state.set_register(a, 1);
                } else {
                    state.set_register(a, 0);
                }
            }
            Opcode::And => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                state.set_register(a, b & c);
            }
            Opcode::Or => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                state.set_register(a, b | c);
            }
            Opcode::Not => {
                let values = state.read_many(2);
                let a = values[0];
                let b = state.resolve_value(values[1]);

                state.set_register(a, !b & 32767);
            }
            Opcode::Set => {
                let values = state.read_many(2);
                let a = values[0];
                let b = values[1];

                state.set_register(a, state.resolve_value(b));
            }
            Opcode::Jmp => {
                let to = state.read_next();
                state.jump_to(to);
            }
            Opcode::JmpIfTrue => {
                let values = state.read_many(2);
                let a = values[0];
                let b = values[1];

                if state.resolve_value(a) != 0 {
                    state.jump_to(state.resolve_value(b));
                }
            }
            Opcode::JmpIfFalse => {
                let values = state.read_many(2);
                let a = values[0];
                let b = values[1];

                if state.resolve_value(a) == 0 {
                    state.jump_to(state.resolve_value(b));
                }
            }
            Opcode::Call => {
                let value = state.read_next();
                let a = state.resolve_value(value);

                state.push(state.ip as u16);
                state.jump_to(a);
            }
            Opcode::RMem => {
                let values = state.read_many(2);
                let a = values[0];
                let b = state.resolve_value(values[1]);

                state.set_register(a, state.read_mem(b));
            }
            Opcode::WMem => {
                let values = state.read_many(2);
                let a = state.resolve_value(values[0]);
                let b = state.resolve_value(values[1]);

                state.write_mem(a, b);
            }
            Opcode::Ret => {
                if let Some(target) = state.pop() {
                  state.jump_to(target);
                } else {
                  break;
                }
            }
            Opcode::In => {
              if !state.is_buffering_string() {
                let mut text = String::new();
                std::io::stdin().read_line(&mut text).unwrap();
                state.start_buffering_string(text);
              }

              let value = state.text_buffer_char();
              let target = state.read_next();
              state.set_register(target, value);
            }
            Opcode::Out => {
                let byte = state.read_next();
                let byte = state.resolve_value(byte) as u8;
                let char = byte as char;
                print!("{}", char);
            }
            Opcode::Noop => {
                state.jump_by(1);
            }
        }
    }

    println!("Execution complete.");
}
