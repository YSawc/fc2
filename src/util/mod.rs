pub fn combine_high_low(l_data: u8, h_data: u8) -> u16 {
    (((h_data as u16) << 8) | l_data as u16) as u16
}

pub fn calc_scrolled_tile_ratio(scrolled_count: u16) -> (u32, u32) {
    let data = scrolled_count % 8;
    return ((8 - data) as u32, data as u32);
}
