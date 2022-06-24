pub fn char_to_bool(c: &char) -> bool {
    match c {
        '0' => false,
        '1' => true,
        _ => unimplemented!(),
    }
}

pub fn chars_to_u32(v: &Vec<char>) -> u32 {
    let mut r = 0;
    for n in v {
        r += n.to_digit(10).unwrap();
    }
    r
}

pub fn ex_plus_ignoring_overflow(l: u8, r: u8) -> u8 {
    if l.checked_add(r).is_none() {
        let l = l as u16;
        let r = r as u16;
        (l + r - (u8::MAX as u16)) as u8
    } else {
        l + r
    }
}

pub fn ex_minus_ignoring_overflow(l: u8, r: u8) -> u8 {
    if l < r {
        r - l
    } else {
        l - r
    }
}

pub fn combine_high_low(l: u8, h: u8) -> u16 {
    (((h as u16) << 8) | l as u16) as u16
}

pub fn bool_to_n(b: bool) -> u8 {
    match b {
        true => 1,
        false => 0,
    }
}

pub fn n_to_bool(n: u32) -> bool {
    match n {
        1 => true,
        0 => false,
        _ => unimplemented!(),
    }
}
