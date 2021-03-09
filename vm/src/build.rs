use std::fs::File;
use std::io::Read;

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
  
    instructions
}