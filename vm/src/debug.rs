use super::exec::{Instruction, State};
use super::opcode::Opcode;
use super::util;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Statistics {
    calls: HashMap<u16, usize>,
    instructions: usize,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            calls: HashMap::new(),
            instructions: 0
        }
    }

    pub fn record_call(&mut self, n: u16) {
        let entry = self.calls.entry(n).or_insert(0);
        *entry += 1;
    }

    pub fn record_instruction(&mut self) {
        self.instructions += 1;
    }
}

pub struct Debugger {
    pub breakpoints: HashSet<u16>,
    pub enabled: bool,
    pub labels: HashMap<usize, String>,
    pub stats: Statistics,
}

impl Debugger {
    pub fn build() -> Debugger {
        let breakpoints = vec![];
        // let breakpoints = vec![2125];
        // let breakpoints = vec![1458];
        // let breakpoints = vec![1841];
        // let breakpoints = vec![2950, 2952];

        let labels = vec![
            (1458, "Map fn impl. r0: string memloc, r1: mapping routine. ret: r1 = 0 if early-exit (routine returns 32767)"),
            (1543, "Map fn. r0: string memloc, r1: mapping routine. ret: r0 = 32767 on succ, r0 = r2 otherwise"),
            (1605, "Char matcher fn. r0: incoming char, r2: needle. ret: r1 = 23767 if a match is found."),
            (1605, "Char matcher impl. r0: incoming char, r2: needle. ret: r1 = 23767 if a match is found."),
            (2125, "Decoder logic. r0: encoded char, r1: key. ret: r0 = decoded char."),
            (2826, "Main loop (?)")
        ];

        let labels = labels.into_iter().map(|(k, v)| (k as usize, v.to_owned()));

        Debugger {
            breakpoints: HashSet::from_iter(breakpoints.into_iter()),
            enabled: false,
            labels: HashMap::from_iter(labels),
            stats: Statistics::new(),
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
                    instruction.disassemble(state.ip, &self.labels)
                );

                let options = [
                    "  (s): Step",
                    "(m): Dump Memory",
                    "(b <addr>): Add breakpoint",
                    "(cstat): Clear statistics",
                    "(stat): Dump Statistics",
                    "(mc <key>): Dump Memory (Coded)",
                    "(ip <value>): Set IP",
                    "(e): Dump Registers",
                    "(g <index> <value>): Set Register",
                    "(t): Dump Stack",
                    "(cs): Dump Call Stack",
                    "(c): Continue",
                    "(w <address> <value>): Write to memory",
                    "(l <count>): Log instructions",
                    "(r <address> <count>): Read from memory",
                ]
                .join("\n  ");

                eprint!("{}\nMake a choice: ", options);

                let mut response = String::new();
                std::io::stdin().read_line(&mut response).unwrap();
                let response = response.trim().split_ascii_whitespace().collect::<Vec<_>>();

                match response[0] {
                    "s" => {
                        break;
                    }
                    "b" => {
                        let address = response[1].parse::<u16>().unwrap();
                        self.breakpoints.insert(address);
                    }
                    "cstat" => {
                        self.stats = Statistics::new();
                    }
                    "stat" => {
                        eprintln!("{:?}", self.stats);
                    }
                    "ip" => {
                        let address = response[1].parse::<usize>().unwrap();
                        state.ip = address;
                        return;
                    }
                    "e" => {
                        eprintln!(
                            "{:?}",
                            state.registers.iter().enumerate().collect::<Vec<_>>()
                        );
                    }
                    "cs" => {
                        let stack = state
                            .call_stack
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>();
                        eprintln!("{}", stack.join(" "));
                    }
                    "w" => {
                        let address = response[1].parse::<u16>().unwrap();
                        let value = response[2].parse::<u16>().unwrap();
                        state.instructions.insert(address, value);
                    }
                    "g" => {
                        let address = response[1].parse::<usize>().unwrap();
                        let value = response[2].parse::<u16>().unwrap();
                        state.registers.insert(address, value);
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
                            eprintln!(
                                "          {}: {}",
                                ip,
                                instruction.disassemble(ip, &self.labels)
                            );
                        }

                        state.ip = original_ip;
                    }
                    "mc" => {
                        let key = response[1].parse::<u16>().unwrap();
                        let mem: Vec<_> = state.instructions.iter().collect();
                        let mut mem = mem.clone();
                        mem.sort_by_key(|(&i, _)| i);
                        let mem = mem
                            .iter()
                            .map(|(_, &v)| util::maybe_to_ascii_coded(v, key))
                            .collect::<String>();
                        eprintln!("{}", mem);
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
