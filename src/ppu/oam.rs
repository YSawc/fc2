pub type SpriteInfos = Vec<SpriteInfo>;

#[derive(Debug, Clone)]
pub struct PrimaryOAM {
    pub sprite_infos: SpriteInfos,
}

impl Default for PrimaryOAM {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimaryOAM {
    fn new() -> Self {
        let sprite_info = SpriteInfo::default();
        let sprite_infos = vec![sprite_info; 64];
        Self { sprite_infos }
    }

    pub fn push_sprite_info(&mut self, data: &Vec<u8>) {
        let mut tile_index = TileIndex::default();
        tile_index.set(data[1]);
        let mut attr = Attr::default();
        attr.set(data[2]);

        let sprite_info = SpriteInfo {
            pos_y: data[0],
            tile_index,
            attr,
            pos_x: data[3],
        };

        self.sprite_infos.push(sprite_info);
    }

    pub fn set_sprite_infos(&mut self, v: Vec<u8>) {
        let mut sprite_infos: SpriteInfos = vec![];
        for i in 0..63 {
            let sprite_idx = i * 4;
            let mut tile_index = TileIndex::default();
            tile_index.set(v[(sprite_idx + 1) as usize]);
            let mut attr = Attr::default();
            attr.set(v[(sprite_idx + 2) as usize]);

            let sprite_info = SpriteInfo {
                pos_y: v[(sprite_idx + 0) as usize],
                tile_index,
                attr,
                pos_x: v[(sprite_idx + 3) as usize],
            };
            sprite_infos.push(sprite_info);
        }
        self.sprite_infos = sprite_infos;
        // print!("[[sprite_infos: {:?}]], ", self.sprite_infos);
    }
}

#[derive(Debug, Clone)]
pub struct SpriteInfo {
    pub pos_y: u8,
    pub tile_index: TileIndex,
    pub attr: Attr,
    pub pos_x: u8,
}

impl Default for SpriteInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SpriteInfo {
    fn new() -> Self {
        Self {
            pos_y: 0,
            tile_index: TileIndex::default(),
            attr: Attr::default(),
            pos_x: 0,
        }
    }

    pub fn in_drawing_range(&self, y: u8) -> bool {
        (self.pos_y <= y) && (self.pos_y + 7 >= y)
    }
}

#[derive(Debug, Clone)]
pub struct TileIndex {
    pub tile_number: u8,
    pub bank_of_tile: bool,
}

impl Default for TileIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl TileIndex {
    fn new() -> Self {
        Self {
            tile_number: 0,
            bank_of_tile: false,
        }
    }

    fn set(&mut self, n: u8) {
        self.tile_number = n;
    }
}

#[derive(Debug, Clone)]
pub struct Attr {
    pub flip_sprite_vertically: bool,
    pub flip_sprite_horizontally: bool,
    pub priority: bool,
    pub unimplemented: u8,
    pub palette: u8,
}

impl Default for Attr {
    fn default() -> Self {
        Self::new()
    }
}

impl Attr {
    fn new() -> Self {
        Self {
            flip_sprite_vertically: false,
            flip_sprite_horizontally: false,
            priority: false,
            unimplemented: 0,
            palette: 0,
        }
    }

    fn set(&mut self, n: u8) {
        self.flip_sprite_vertically = (n & 0b10000000) != 0;
        self.flip_sprite_horizontally = (n & 0b01000000) != 0;
        self.priority = (n & 0b00100000) != 0;
        self.unimplemented = n & 0b00011100;
        self.palette = n & 0b00000011;
    }
}

#[derive(Debug, Clone)]
pub struct SecondaryOAM {
    pub sprite_infos: SpriteInfos,
}

impl Default for SecondaryOAM {
    fn default() -> Self {
        Self::new()
    }
}

impl SecondaryOAM {
    fn new() -> Self {
        let sprite_info = SpriteInfo::default();
        let sprite_infos = vec![sprite_info; 8];
        Self { sprite_infos }
    }

    pub fn clear_sprite_infos(&mut self) {
        let sprite_infos = vec![];
        self.sprite_infos = sprite_infos;
    }

    pub fn pick_sprite_info_with_x(&mut self, x: u8) -> Option<&SpriteInfo> {
        let sprite_info = self
            .sprite_infos
            .iter()
            .find(|sprite_info| sprite_info.pos_x == x);
        return sprite_info;
    }
}
