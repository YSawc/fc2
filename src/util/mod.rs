pub fn combine_high_low(l: u8, h: u8) -> u16 {
    (((h as u16) << 8) | l as u16) as u16
}

pub fn calc_scroll_x_left_right_ratio_per_tile(scrolled_x: u8) -> (u32, u32) {
    let n = scrolled_x % 8;
    return ((8 - n) as u32, n as u32);
}
