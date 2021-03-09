mod build;
mod debug;
mod exec;
mod opcode;
mod util;

pub fn start(filename: &str) {
    let instructions = build::read_binary(filename);
    // println!("{:?}", instructions);
    exec::run_loop(instructions);
}
