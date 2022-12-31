pub fn combine_high_low(l: u8, h: u8) -> u16 {
    (((h as u16) << 8) | l as u16) as u16
}

pub fn calc_scrolled_tile_ratio(scrolled_count: u16) -> (u32, u32) {
    let n = scrolled_count % 8;
    return ((8 - n) as u32, n as u32);
}
