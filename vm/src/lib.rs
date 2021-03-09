mod build;
mod debug;
mod exec;
mod util;
mod opcode;

pub fn start(filename: &str) {
    let instructions = build::read_binary(filename);
    // println!("{:?}", instructions);
    exec::run_loop(instructions);
}