mod build;
mod debug;
mod exec;
mod opcode;
mod util;
mod disasm;

pub fn start(filename: &str) {
    let instructions = build::read_binary(filename);
    // println!("{:?}", instructions);
    exec::run_loop(instructions);
}

pub fn export(filename: &str) {
    let instructions = build::read_binary(filename);
    disasm::disassemble_instructions(instructions);
}
