use super::exec::{Instruction, State};
use super::opcode::Opcode;
use super::util;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Debugger {
    pub breakpoints: HashSet<u16>,
    pub enabled: bool,
}

impl Debugger {
    pub fn build() -> Debugger {
        let breakpoints = vec![];

        Debugger {
            breakpoints: HashSet::from_iter(breakpoints.iter().cloned()),
            enabled: false,
        }
    }

    pub fn check_for_breakpoints(&mut self, ip: usize) {
        if self.breakpoints.contains(&(ip as u16)) {
            self.enabled = true;
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn maybe_present(&mut self, state: &mut State, instruction: &Instruction) {
        if self.enabled {
            let original_ip = state.ip.clone();
            state.ip -= instruction.args.len() + 1;

            loop {
                eprintln!(
                    "\nCurrent Instruction: {}: {}",
                    state.ip,
                    instruction.disassemble()
                );

                eprint!("  (s): Step\n  (m): Dump Mem\n  (t): Dump Stack\n  (c): Continue\n  (w <address> <value>): Write to memory\n  (l <count>): Log instructions\n  (r <addr> <count>): Read from memory\nMake a choice: ");
                let mut response = String::new();
                std::io::stdin().read_line(&mut response).unwrap();
                let response = response.trim().split_ascii_whitespace().collect::<Vec<_>>();

                match response[0] {
                    "s" => {
                        break;
                    }
                    "w" => {
                        let address = response[1].parse::<u16>().unwrap();
                        let value = response[2].parse::<u16>().unwrap();
                        state.instructions.insert(address, value);
                    }
                    "r" => {
                        let address = response[1].parse::<u16>().unwrap();
                        let count = response[2].parse::<u16>().unwrap();

                        for i in address..address + count {
                            if let Some(instruction) = state.instructions.get(&i) {
                                eprintln!(
                                    "{}: {} / {}",
                                    i,
                                    instruction,
                                    util::maybe_to_ascii(*instruction)
                                );
                            } else {
                                eprintln!("{}: <BLANK>", i);
                            }
                        }
                    }
                    "l" => {
                        let original_ip = state.ip.clone();
                        let count = if response.len() == 2 {
                            response[1].parse::<u16>().unwrap()
                        } else {
                            5
                        };
                        let count = count as usize;

                        for _ in 0..count {
                            let ip = state.ip.clone();
                            let opcode = Opcode::from(state.instructions[&(state.ip as u16)]);
                            let instruction = Instruction::build(opcode, state);
                            eprintln!("          {}: {}", ip, instruction.disassemble());
                        }

                        state.ip = original_ip;
                    }
                    "m" => {
                        let mem: Vec<_> = state.instructions.iter().collect();
                        let mut mem = mem.clone();
                        mem.sort_by_key(|(&i, _)| i);
                        let mem = mem
                            .iter()
                            .map(|(_, &v)| util::maybe_to_ascii(v))
                            .collect::<String>();
                        eprintln!("{}", mem);
                    }
                    "t" => {
                        let stack = state
                            .stack
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>();
                        eprintln!("{}", stack.join(" "));
                    }
                    "c" => {
                        self.enabled = false;
                        break;
                    }
                    _ => panic!("Invalid response"),
                };
            }

            state.ip = original_ip;
        }
    }
}
