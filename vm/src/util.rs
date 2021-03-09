pub fn maybe_to_ascii(n: u16) -> char {
  if n < 128 {
      n as u8 as char
  } else {
      '.'
  }
}