#[derive(Debug)]
pub enum Opcode {
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
    In,
}
impl std::fmt::Display for Opcode {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{:?}", self)
  }
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