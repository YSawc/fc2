#[derive(Debug, Clone)]
pub struct Map {
    pub pattern_table_00: [u8; 0x1000],
    pub pattern_table_01: [u8; 0x1000],
    pub name_table_00: [u8; 0x03C0],
    pub attr_table_00: [u8; 0x0040],
    pub name_table_01: [u8; 0x03C0],
    pub attr_table_01: [u8; 0x0040],
    pub name_table_02: [u8; 0x03C0],
    pub attr_table_02: [u8; 0x0040],
    pub name_table_03: [u8; 0x03C0],
    pub attr_table_03: [u8; 0x0040],
    pub name_and_attr_table_mirror: [u8; 0x0EFF],
    pub background_table: [u8; 0x0010],
    pub sprite_pallet: [u8; 0x0010],
    pub background_and_sprite_pallet_mirror: [u8; 0x00DF],
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Self {
        Self {
            pattern_table_00: [0; 0x1000],
            pattern_table_01: [0; 0x1000],
            name_table_00: [0; 0x03C0],
            attr_table_00: [0; 0x0040],
            name_table_01: [0; 0x03C0],
            attr_table_01: [0; 0x0040],
            name_table_02: [0; 0x03C0],
            attr_table_02: [0; 0x0040],
            name_table_03: [0; 0x03C0],
            attr_table_03: [0; 0x0040],
            name_and_attr_table_mirror: [0; 0x0EFF],
            background_table: [0; 0x0010],
            sprite_pallet: [0; 0x0010],
            background_and_sprite_pallet_mirror: [0; 0x00DF],
        }
    }

    pub fn addr(&self, n: u16) -> u8 {
        match n {
            0x0000..=0x0FFF => self.pattern_table_00[n as usize],
            0x1000..=0x1FFF => self.pattern_table_01[(n - 0x1000) as usize],
            0x2000..=0x23BF => self.name_table_00[(n - 0x2000) as usize],
            0x23C0..=0x23FF => self.attr_table_00[(n - 0x23C0) as usize],
            0x2400..=0x27BF => self.name_table_01[(n - 0x2400) as usize],
            0x27C0..=0x27FF => self.attr_table_01[(n - 0x27C0) as usize],
            0x2800..=0x2BBF => self.name_table_02[(n - 0x2800) as usize],
            0x2BC0..=0x2BFF => self.attr_table_02[(n - 0x2BC0) as usize],
            0x2C00..=0x2FBF => self.name_table_03[(n - 0x2C00) as usize],
            0x2FC0..=0x2FFF => self.attr_table_03[(n - 0x2FC0) as usize],
            0x3000..=0x3EFF => self.name_and_attr_table_mirror[(n - 0x3000) as usize],
            0x3F00..=0x3F0F => self.background_table[(n - 0x3F00) as usize],
            0x3F10..=0x3F1F => self.sprite_pallet[(n - 0x3F10) as usize],
            0x3F20..=0x3FFF => self.background_and_sprite_pallet_mirror[(n - 0x3F20) as usize],
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, n: u16, r: u8) {
        match n {
            0x0000..=0x0FFF => self.pattern_table_00[n as usize] = r,
            0x1000..=0x1FFF => self.pattern_table_01[(n - 0x1000) as usize] = r,
            0x2000..=0x23BF => self.name_table_00[(n - 0x2000) as usize] = r,
            0x23C0..=0x23FF => self.attr_table_00[(n - 0x23C0) as usize] = r,
            0x2400..=0x27BF => self.name_table_01[(n - 0x2400) as usize] = r,
            0x27C0..=0x27FF => self.attr_table_01[(n - 0x27C0) as usize] = r,
            0x2800..=0x2BBF => self.name_table_02[(n - 0x2800) as usize] = r,
            0x2BC0..=0x2BFF => self.attr_table_02[(n - 0x2BC0) as usize] = r,
            0x2C00..=0x2FBF => self.name_table_03[(n - 0x2C00) as usize] = r,
            0x2FC0..=0x2FFF => self.attr_table_03[(n - 0x2FC0) as usize] = r,
            0x3000..=0x3EFF => self.name_and_attr_table_mirror[(n - 0x3000) as usize] = r,
            0x3F00..=0x3F0F => self.background_table[(n - 0x3F00) as usize] = r,
            0x3F10..=0x3F1F => self.sprite_pallet[(n - 0x3F10) as usize] = r,
            0x3F20..=0x3FFF => self.background_and_sprite_pallet_mirror[(n - 0x3F20) as usize] = r,
            _ => unreachable!(),
        };
    }
}
