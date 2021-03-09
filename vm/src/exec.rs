use super::debug::Debugger;
use super::opcode::Opcode;

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

pub struct Instruction {
    pub opcode: Opcode,
    pub unresolved_args: Vec<u16>,
    pub args: Vec<u16>,
}

impl Instruction {
    pub fn build(opcode: Opcode, state: &mut State) -> Instruction {
        match opcode {
            Opcode::Halt => Instruction {
                opcode,
                args: vec![],
                unresolved_args: vec![],
            },
            Opcode::Add => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);
                Instruction {
                    opcode,
                    args: vec![a, b, c],
                    unresolved_args: values,
                }
            }
            Opcode::Mult => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b, c],
                }
            }
            Opcode::Mod => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b, c],
                }
            }
            Opcode::Push => {
                let value = state.read_next();
                let a = state.resolve_value(value);

                Instruction {
                    opcode,
                    unresolved_args: vec![value],
                    args: vec![a],
                }
            }
            Opcode::Pop => {
                let a = state.read_next();

                Instruction {
                    opcode,
                    unresolved_args: vec![a],
                    args: vec![a],
                }
            }
            Opcode::Gt => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b, c],
                }
            }
            Opcode::Eq => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b, c],
                }
            }
            Opcode::And => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b, c],
                }
            }
            Opcode::Or => {
                let values = state.read_many(3);
                let a = values[0];
                let b = state.resolve_value(values[1]);
                let c = state.resolve_value(values[2]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b, c],
                }
            }
            Opcode::Not => {
                let values = state.read_many(2);
                let a = values[0];
                let b = state.resolve_value(values[1]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b],
                }
            }
            Opcode::Set => {
                let values = state.read_many(2);
                let a = values[0];
                let b = state.resolve_value(values[1]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b],
                }
            }
            Opcode::Jmp => {
                let to = state.read_next();

                Instruction {
                    opcode,
                    unresolved_args: vec![to],
                    args: vec![to],
                }
            }
            Opcode::JmpIfTrue => {
                let values = state.read_many(2);
                let a = state.resolve_value(values[0]);
                let b = state.resolve_value(values[1]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b],
                }
            }
            Opcode::JmpIfFalse => {
                let values = state.read_many(2);
                let a = state.resolve_value(values[0]);
                let b = state.resolve_value(values[1]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b],
                }
            }
            Opcode::Call => {
                let value = state.read_next();
                let a = state.resolve_value(value);

                Instruction {
                    opcode,
                    unresolved_args: vec![value],
                    args: vec![a],
                }
            }
            Opcode::RMem => {
                let values = state.read_many(2);
                let a = values[0];
                let b = state.resolve_value(values[1]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b],
                }
            }
            Opcode::WMem => {
                let values = state.read_many(2);
                let a = state.resolve_value(values[0]);
                let b = state.resolve_value(values[1]);

                Instruction {
                    opcode,
                    unresolved_args: values,
                    args: vec![a, b],
                }
            }
            Opcode::Ret => Instruction {
                opcode,
                unresolved_args: vec![],
                args: vec![],
            },
            Opcode::In => {
                let a = state.read_next();

                Instruction {
                    opcode,
                    unresolved_args: vec![a],
                    args: vec![a],
                }
            }
            Opcode::Out => {
                let byte = state.read_next();
                let resolved_byte = state.resolve_value(byte);

                Instruction {
                    opcode,
                    unresolved_args: vec![byte],
                    args: vec![resolved_byte],
                }
            }
            Opcode::Noop => Instruction {
                opcode,
                unresolved_args: vec![],
                args: vec![],
            },
        }
    }

    fn register_pretty(&self, s: &u16) -> String {
        match s {
            32768 => "r0".to_owned(),
            32769 => "r1".to_owned(),
            32770 => "r2".to_owned(),
            32771 => "r3".to_owned(),
            32772 => "r4".to_owned(),
            32773 => "r5".to_owned(),
            32774 => "r6".to_owned(),
            32775 => "r7".to_owned(),
            _ => s.to_string(),
        }
    }

    pub fn disassemble(&self) -> String {
        format!(
            "{} / {} / {}",
            self.opcode.to_string(),
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.unresolved_args
                .iter()
                .map(|n| self.register_pretty(n))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub struct State {
    pub instructions: HashMap<u16, u16>,
    pub registers: Vec<u16>,
    pub ip: usize,
    pub stack: Vec<u16>,
    pub text_buffer: Option<String>,
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
            text_buffer: None,
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

    fn text_buffer_char(&mut self) -> u16 {
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

    fn dump(&self) {
        let mut s = String::new();

        let instructions: Vec<_> = self.instructions.iter().collect();
        let mut instructions = instructions.clone();
        instructions.sort_by_key(|(&i, _)| i);

        s.push_str("IP\n");
        s.push_str(&self.ip.to_string());
        s.push_str("\n");
        s.push_str(&self.instructions[&(self.ip as u16)].to_string());

        s.push_str("\n\nRegisters\n");
        s.push_str(serde_json::to_string(&self.registers).unwrap().as_ref());

        s.push_str("\n\nStack\n");
        s.push_str(serde_json::to_string(&self.stack).unwrap().as_ref());

        s.push_str("\n\nMemory\n");
        s.push_str(serde_json::to_string(&instructions).unwrap().as_ref());

        std::fs::write("dump.txt", s).unwrap();

        eprintln!("Dumped");
    }
}

pub fn run_loop(instructions: Vec<u16>) {
    let mut state = State::build(instructions);
    let mut debugger = Debugger::build();

    loop {
        debugger.check_for_breakpoints(state.ip);

        let instruction = Instruction::build(state.opcode(), &mut state);

        if !state.is_buffering_string() {
            debugger.maybe_present(&mut state, &instruction);
        }

        match instruction.opcode {
            Opcode::Halt => {
                break;
            }
            Opcode::Add => {
                if let [a, b, c] = instruction.args.as_slice() {
                    let result = ((*b as usize + *c as usize) % 32768) as u16;
                    state.set_register(*a, result);
                }
            }
            Opcode::Mult => {
                if let [a, b, c] = instruction.args.as_slice() {
                    let result = ((*b as usize * *c as usize) % 32768) as u16;
                    state.set_register(*a, result);
                }
            }
            Opcode::Mod => {
                if let [a, b, c] = instruction.args.as_slice() {
                    let result = *b % *c;
                    state.set_register(*a, result);
                }
            }
            Opcode::Push => {
                if let [a] = instruction.args.as_slice() {
                    state.push(*a);
                }
            }
            Opcode::Pop => {
                if let [a] = instruction.args.as_slice() {
                    let value = state.pop().unwrap();
                    state.set_register(*a, value);
                }
            }
            Opcode::Gt => {
                if let [a, b, c] = instruction.args.as_slice() {
                    if *b > *c {
                        state.set_register(*a, 1);
                    } else {
                        state.set_register(*a, 0);
                    }
                }
            }
            Opcode::Eq => {
                if let [a, b, c] = instruction.args.as_slice() {
                    if *b == *c {
                        state.set_register(*a, 1);
                    } else {
                        state.set_register(*a, 0);
                    }
                }
            }
            Opcode::And => {
                if let [a, b, c] = instruction.args.as_slice() {
                    state.set_register(*a, *b & *c);
                }
            }
            Opcode::Or => {
                if let [a, b, c] = instruction.args.as_slice() {
                    state.set_register(*a, *b | *c);
                }
            }
            Opcode::Not => {
                if let [a, b] = instruction.args.as_slice() {
                    state.set_register(*a, !*b & 32767);
                }
            }
            Opcode::Set => {
                if let [a, b] = instruction.args.as_slice() {
                    state.set_register(*a, *b);
                }
            }
            Opcode::Jmp => {
                if let [to] = instruction.args.as_slice() {
                    state.jump_to(*to);
                }
            }
            Opcode::JmpIfTrue => {
                if let [a, b] = instruction.args.as_slice() {
                    if *a != 0 {
                        state.jump_to(*b);
                    }
                }
            }
            Opcode::JmpIfFalse => {
                if let [a, b] = instruction.args.as_slice() {
                    if *a == 0 {
                        state.jump_to(*b);
                    }
                }
            }
            Opcode::Call => {
                if let [a] = instruction.args.as_slice() {
                    state.push(state.ip as u16);
                    state.jump_to(*a);
                }
            }
            Opcode::RMem => {
                if let [a, b] = instruction.args.as_slice() {
                    state.set_register(*a, state.read_mem(*b));
                }
            }
            Opcode::WMem => {
                if let [a, b] = instruction.args.as_slice() {
                    state.write_mem(*a, *b);
                }
            }
            Opcode::Ret => {
                if let Some(target) = state.pop() {
                    state.jump_to(target);
                } else {
                    break;
                }
            }
            Opcode::In => {
                if let [a] = instruction.args.as_slice() {
                    if !state.is_buffering_string() {
                        let mut text = String::new();
                        std::io::stdin().read_line(&mut text).unwrap();

                        if text == "dump\n" {
                            state.dump();
                            continue;
                        }

                        if text == "debug\n" {
                            eprintln!("Starting debugger...");
                            debugger.enable();
                            // Rewind so this `In` is replayed
                            state.ip -= 2;
                            continue;
                        }

                        state.start_buffering_string(text);
                    }

                    let value = state.text_buffer_char();
                    state.set_register(*a, value);
                }
            }
            Opcode::Out => {
                if let [a] = instruction.args.as_slice() {
                    if !debugger.enabled && *a == 70 as u16 {
                        debugger.enable();
                        state.ip -= 2;
                        continue;
                    }

                    let byte = *a as u8;
                    let char = byte as char;
                    print!("{}", char);
                }
            }
            Opcode::Noop => {
                state.jump_by(1);
            }
        }
    }

    println!("Execution complete.");
}
