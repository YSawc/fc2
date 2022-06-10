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
