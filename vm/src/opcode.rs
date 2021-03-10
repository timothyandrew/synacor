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
    Unknown
}
impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Opcode {
    pub fn arg_count(&self) -> usize {
        match self {
            Opcode::Halt => 0,
            Opcode::Set => 2,
            Opcode::Push => 1,
            Opcode::Pop => 1,
            Opcode::Eq => 3,
            Opcode::Gt => 3,
            Opcode::Jmp => 1,
            Opcode::JmpIfTrue => 2,
            Opcode::JmpIfFalse => 2,
            Opcode::Add => 3,
            Opcode::Mult => 3,
            Opcode::Mod => 3,
            Opcode::And => 3,
            Opcode::Or => 3,
            Opcode::Not => 2,
            Opcode::RMem => 2,
            Opcode::WMem => 2,
            Opcode::Call => 1,
            Opcode::Ret => 0,
            Opcode::Out => 1,
            Opcode::In => 1,
            Opcode::Noop => 0,
            Opcode::Unknown => 0,
        }
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
            _ => Opcode::Unknown,
        }
    }
}
