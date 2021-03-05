fn main() {
    let filename = "/Users/tim/dev/synacor/challenge.bin";
    let instructions = vm::read_binary(filename);
    // println!("{:?}", instructions);
    vm::run_loop(instructions);
}
