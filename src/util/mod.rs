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

pub fn combine_high_low(l: u8, h: u8) -> u16 {
    (((h as u16) << 8) | l as u16) as u16
}

pub fn calc_scroll_x_left_right_ratio_per_tile(scrolled_x: u8) -> (u8, u8) {
    let n = scrolled_x % 8;
    return (8 - n, n);
}
