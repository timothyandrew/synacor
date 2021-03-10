pub fn to_ascii(n: u16) -> Option<char> {
    if n < 128 {
        Some(n as u8 as char)
    } else {
        None
    }
}

pub fn maybe_to_ascii(n: u16) -> char {
    if n < 128 {
        n as u8 as char
    } else {
        '.'
    }
}

pub fn maybe_to_ascii_coded(n: u16, key: u16) -> char {
  let r0 = n;
  let r1 = key;

  let r2 = r0 & r1;
  let r2 = !r2;
  let r0 = r0 | r1;
  let r0 = r0 & r2;

  return maybe_to_ascii(r0);
}

pub fn register_pretty(s: &u16) -> String {
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
