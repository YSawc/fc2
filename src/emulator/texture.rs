use super::configure::SQUARE_SIZE;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub struct TextureBuffer<const PLAYGROUND_WIDTH: u32, const VISIBLE_LINES: u16> {
    pub buffer: [u8; 184320],
    colors: [[u8; 3]; 64],
}

impl<const PLAYGROUND_WIDTH: u32, const VISIBLE_LINES: u16>
    TextureBuffer<PLAYGROUND_WIDTH, VISIBLE_LINES>
{
    pub fn new() -> Self {
        let buffer: [u8; 184320] = [0; 184320];

        let colors = [
            [0x80, 0x80, 0x80],
            [0x00, 0x3D, 0xA6],
            [0x00, 0x12, 0xB0],
            [0x44, 0x00, 0x96],
            [0xA1, 0x00, 0x5E],
            [0xC7, 0x00, 0x28],
            [0xBA, 0x06, 0x00],
            [0x8C, 0x17, 0x00],
            [0x5C, 0x2F, 0x00],
            [0x10, 0x45, 0x00],
            [0x05, 0x4A, 0x00],
            [0x00, 0x47, 0x2E],
            [0x00, 0x41, 0x66],
            [0x00, 0x00, 0x00],
            [0x05, 0x05, 0x05],
            [0x05, 0x05, 0x05],
            [0xC7, 0xC7, 0xC7],
            [0x00, 0x77, 0xFF],
            [0x21, 0x55, 0xFF],
            [0x82, 0x37, 0xFA],
            [0xEB, 0x2F, 0xB5],
            [0xFF, 0x29, 0x50],
            [0xFF, 0x22, 0x00],
            [0xD6, 0x32, 0x00],
            [0xC4, 0x62, 0x00],
            [0x35, 0x80, 0x00],
            [0x05, 0x8F, 0x00],
            [0x00, 0x8A, 0x55],
            [0x00, 0x99, 0xCC],
            [0x21, 0x21, 0x21],
            [0x09, 0x09, 0x09],
            [0x09, 0x09, 0x09],
            [0xFF, 0xFF, 0xFF],
            [0x0F, 0xD7, 0xFF],
            [0x69, 0xA2, 0xFF],
            [0xD4, 0x80, 0xFF],
            [0xFF, 0x45, 0xF3],
            [0xFF, 0x61, 0x8B],
            [0xFF, 0x88, 0x33],
            [0xFF, 0x9C, 0x12],
            [0xFA, 0xBC, 0x20],
            [0x9F, 0xE3, 0x0E],
            [0x2B, 0xF0, 0x35],
            [0x0C, 0xF0, 0xA4],
            [0x05, 0xFB, 0xFF],
            [0x5E, 0x5E, 0x5E],
            [0x0D, 0x0D, 0x0D],
            [0x0D, 0x0D, 0x0D],
            [0xFF, 0xFF, 0xFF],
            [0xA6, 0xFC, 0xFF],
            [0xB3, 0xEC, 0xFF],
            [0xDA, 0xAB, 0xEB],
            [0xFF, 0xA8, 0xF9],
            [0xFF, 0xAB, 0xB3],
            [0xFF, 0xD2, 0xB0],
            [0xFF, 0xEF, 0xA6],
            [0xFF, 0xF7, 0x9C],
            [0xD7, 0xE8, 0x95],
            [0xA6, 0xED, 0xAF],
            [0xA2, 0xF2, 0xDA],
            [0x99, 0xFF, 0xFC],
            [0xDD, 0xDD, 0xDD],
            [0x11, 0x11, 0x11],
            [0x11, 0x11, 0x11],
        ];

        Self { buffer, colors }
    }

    fn pick_offset(&self, x: u8, y: u8) -> usize {
        let pitch = PLAYGROUND_WIDTH as usize * 8 * 3;
        let offset = (y as usize) * pitch + x as usize * 3;
        offset as usize
    }

    pub fn insert_color(&mut self, x: u8, y: u8, colors_idx: usize) {
        let offset = self.pick_offset(x, y);
        let color = self.colors[colors_idx];
        for n in 0..3 as usize {
            self.buffer[offset + n] = color[n];
        }
    }
}

pub fn texture_combine_builtin_colors<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<[Texture<'a>; 56], String> {
    let colors = [
        [0x80, 0x80, 0x80],
        [0x00, 0x3D, 0xA6],
        [0x00, 0x12, 0xB0],
        [0x44, 0x00, 0x96],
        [0xA1, 0x00, 0x5E],
        [0xC7, 0x00, 0x28],
        [0xBA, 0x06, 0x00],
        [0x8C, 0x17, 0x00],
        [0x5C, 0x2F, 0x00],
        [0x10, 0x45, 0x00],
        [0x05, 0x4A, 0x00],
        [0x00, 0x47, 0x2E],
        [0x00, 0x41, 0x66],
        [0x00, 0x00, 0x00],
        [0x05, 0x05, 0x05],
        [0x05, 0x05, 0x05],
        [0xC7, 0xC7, 0xC7],
        [0x00, 0x77, 0xFF],
        [0x21, 0x55, 0xFF],
        [0x82, 0x37, 0xFA],
        [0xEB, 0x2F, 0xB5],
        [0xFF, 0x29, 0x50],
        [0xFF, 0x22, 0x00],
        [0xD6, 0x32, 0x00],
        [0xC4, 0x62, 0x00],
        [0x35, 0x80, 0x00],
        [0x05, 0x8F, 0x00],
        [0x00, 0x8A, 0x55],
        [0x00, 0x99, 0xCC],
        [0x21, 0x21, 0x21],
        [0x09, 0x09, 0x09],
        [0x09, 0x09, 0x09],
        [0xFF, 0xFF, 0xFF],
        [0x0F, 0xD7, 0xFF],
        [0x69, 0xA2, 0xFF],
        [0xD4, 0x80, 0xFF],
        [0xFF, 0x45, 0xF3],
        [0xFF, 0x61, 0x8B],
        [0xFF, 0x88, 0x33],
        [0xFF, 0x9C, 0x12],
        [0xFA, 0xBC, 0x20],
        [0x9F, 0xE3, 0x0E],
        [0x2B, 0xF0, 0x35],
        [0x0C, 0xF0, 0xA4],
        [0x05, 0xFB, 0xFF],
        [0x5E, 0x5E, 0x5E],
        [0x0D, 0x0D, 0x0D],
        [0x0D, 0x0D, 0x0D],
        [0xFF, 0xFF, 0xFF],
        [0xA6, 0xFC, 0xFF],
        [0xB3, 0xEC, 0xFF],
        [0xDA, 0xAB, 0xEB],
        [0xFF, 0xA8, 0xF9],
        [0xFF, 0xAB, 0xB3],
        [0xFF, 0xD2, 0xB0],
        [0xFF, 0xEF, 0xA6],
        [0xFF, 0xF7, 0x9C],
        [0xD7, 0xE8, 0x95],
        [0xA6, 0xED, 0xAF],
        [0xA2, 0xF2, 0xDA],
        [0x99, 0xFF, 0xFC],
        [0xDD, 0xDD, 0xDD],
        [0x11, 0x11, 0x11],
        [0x11, 0x11, 0x11],
    ];

    enum TextureColor {
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
        T17,
        T18,
        T19,
        T20,
        T21,
        T22,
        T23,
        T24,
        T25,
        T26,
        T27,
        T28,
        T29,
        T30,
        T31,
        T32,
        T33,
        T34,
        T35,
        T36,
        T37,
        T38,
        T39,
        T40,
        T41,
        T42,
        T43,
        T44,
        T45,
        T46,
        T47,
        T48,
        T49,
        T50,
        T51,
        T52,
        T53,
        T54,
        T55,
    }
    let mut square_texture0 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture1 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture2 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture3 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture4 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture5 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture6 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture7 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture8 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture9 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture10 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture11 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture12 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture13 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture14 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture15 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture16 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture17 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture18 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture19 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture20 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture21 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture22 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture23 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture24 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture25 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture26 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture27 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture28 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture29 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture30 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture31 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture32 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture33 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture34 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture35 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture36 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture37 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture38 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture39 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture40 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture41 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture42 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture43 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture44 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture45 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture46 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture47 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture48 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture49 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture50 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture51 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture52 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture53 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture54 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture55 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    {
        let textures = vec![
            (&mut square_texture0, TextureColor::T0),
            (&mut square_texture1, TextureColor::T1),
            (&mut square_texture2, TextureColor::T2),
            (&mut square_texture3, TextureColor::T3),
            (&mut square_texture4, TextureColor::T4),
            (&mut square_texture5, TextureColor::T5),
            (&mut square_texture6, TextureColor::T6),
            (&mut square_texture7, TextureColor::T7),
            (&mut square_texture8, TextureColor::T8),
            (&mut square_texture9, TextureColor::T9),
            (&mut square_texture10, TextureColor::T10),
            (&mut square_texture11, TextureColor::T11),
            (&mut square_texture12, TextureColor::T12),
            (&mut square_texture13, TextureColor::T13),
            (&mut square_texture14, TextureColor::T14),
            (&mut square_texture15, TextureColor::T15),
            (&mut square_texture16, TextureColor::T16),
            (&mut square_texture17, TextureColor::T17),
            (&mut square_texture18, TextureColor::T18),
            (&mut square_texture19, TextureColor::T19),
            (&mut square_texture20, TextureColor::T20),
            (&mut square_texture21, TextureColor::T21),
            (&mut square_texture22, TextureColor::T22),
            (&mut square_texture23, TextureColor::T23),
            (&mut square_texture24, TextureColor::T24),
            (&mut square_texture25, TextureColor::T25),
            (&mut square_texture26, TextureColor::T26),
            (&mut square_texture27, TextureColor::T27),
            (&mut square_texture28, TextureColor::T28),
            (&mut square_texture29, TextureColor::T29),
            (&mut square_texture30, TextureColor::T30),
            (&mut square_texture31, TextureColor::T31),
            (&mut square_texture32, TextureColor::T32),
            (&mut square_texture33, TextureColor::T33),
            (&mut square_texture34, TextureColor::T34),
            (&mut square_texture35, TextureColor::T35),
            (&mut square_texture36, TextureColor::T36),
            (&mut square_texture37, TextureColor::T37),
            (&mut square_texture38, TextureColor::T38),
            (&mut square_texture39, TextureColor::T39),
            (&mut square_texture40, TextureColor::T40),
            (&mut square_texture41, TextureColor::T41),
            (&mut square_texture42, TextureColor::T42),
            (&mut square_texture43, TextureColor::T43),
            (&mut square_texture44, TextureColor::T44),
            (&mut square_texture45, TextureColor::T45),
            (&mut square_texture46, TextureColor::T46),
            (&mut square_texture47, TextureColor::T47),
            (&mut square_texture48, TextureColor::T48),
            (&mut square_texture49, TextureColor::T49),
            (&mut square_texture50, TextureColor::T50),
            (&mut square_texture51, TextureColor::T51),
            (&mut square_texture52, TextureColor::T52),
            (&mut square_texture53, TextureColor::T53),
            (&mut square_texture54, TextureColor::T54),
            (&mut square_texture55, TextureColor::T55),
        ];
        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                macro_rules! set_texture {
                    ($id:expr) => {{
                        let r = colors[$id][0];
                        let g = colors[$id][1];
                        let b = colors[$id][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }};
                }
                match *user_context {
                    TextureColor::T0 => set_texture!(0),
                    TextureColor::T1 => set_texture!(1),
                    TextureColor::T2 => set_texture!(2),
                    TextureColor::T3 => set_texture!(3),
                    TextureColor::T4 => set_texture!(4),
                    TextureColor::T5 => set_texture!(5),
                    TextureColor::T6 => set_texture!(6),
                    TextureColor::T7 => set_texture!(7),
                    TextureColor::T8 => set_texture!(8),
                    TextureColor::T9 => set_texture!(9),
                    TextureColor::T10 => set_texture!(10),
                    TextureColor::T11 => set_texture!(11),
                    TextureColor::T12 => set_texture!(12),
                    TextureColor::T13 => set_texture!(13),
                    TextureColor::T14 => set_texture!(14),
                    TextureColor::T15 => set_texture!(15),
                    TextureColor::T16 => set_texture!(16),
                    TextureColor::T17 => set_texture!(17),
                    TextureColor::T18 => set_texture!(18),
                    TextureColor::T19 => set_texture!(19),
                    TextureColor::T20 => set_texture!(20),
                    TextureColor::T21 => set_texture!(21),
                    TextureColor::T22 => set_texture!(22),
                    TextureColor::T23 => set_texture!(23),
                    TextureColor::T24 => set_texture!(24),
                    TextureColor::T25 => set_texture!(25),
                    TextureColor::T26 => set_texture!(26),
                    TextureColor::T27 => set_texture!(27),
                    TextureColor::T28 => set_texture!(28),
                    TextureColor::T29 => set_texture!(29),
                    TextureColor::T30 => set_texture!(30),
                    TextureColor::T31 => set_texture!(31),
                    TextureColor::T32 => set_texture!(32),
                    TextureColor::T33 => set_texture!(33),
                    TextureColor::T34 => set_texture!(34),
                    TextureColor::T35 => set_texture!(35),
                    TextureColor::T36 => set_texture!(36),
                    TextureColor::T37 => set_texture!(37),
                    TextureColor::T38 => set_texture!(38),
                    TextureColor::T39 => set_texture!(39),
                    TextureColor::T40 => set_texture!(40),
                    TextureColor::T41 => set_texture!(41),
                    TextureColor::T42 => set_texture!(42),
                    TextureColor::T43 => set_texture!(43),
                    TextureColor::T44 => set_texture!(44),
                    TextureColor::T45 => set_texture!(45),
                    TextureColor::T46 => set_texture!(46),
                    TextureColor::T47 => set_texture!(47),
                    TextureColor::T48 => set_texture!(48),
                    TextureColor::T49 => set_texture!(49),
                    TextureColor::T50 => set_texture!(50),
                    TextureColor::T51 => set_texture!(51),
                    TextureColor::T52 => set_texture!(52),
                    TextureColor::T53 => set_texture!(53),
                    TextureColor::T54 => set_texture!(54),
                    TextureColor::T55 => set_texture!(55),
                };
            })
            .map_err(|e| e.to_string())?;
    }

    let colors = [
        square_texture0,
        square_texture1,
        square_texture2,
        square_texture3,
        square_texture4,
        square_texture5,
        square_texture6,
        square_texture7,
        square_texture8,
        square_texture9,
        square_texture10,
        square_texture11,
        square_texture12,
        square_texture13,
        square_texture14,
        square_texture15,
        square_texture16,
        square_texture17,
        square_texture18,
        square_texture19,
        square_texture20,
        square_texture21,
        square_texture22,
        square_texture23,
        square_texture24,
        square_texture25,
        square_texture26,
        square_texture27,
        square_texture28,
        square_texture29,
        square_texture30,
        square_texture31,
        square_texture32,
        square_texture33,
        square_texture34,
        square_texture35,
        square_texture36,
        square_texture37,
        square_texture38,
        square_texture39,
        square_texture40,
        square_texture41,
        square_texture42,
        square_texture43,
        square_texture44,
        square_texture45,
        square_texture46,
        square_texture47,
        square_texture48,
        square_texture49,
        square_texture50,
        square_texture51,
        square_texture52,
        square_texture53,
        square_texture54,
        square_texture55,
    ];

    Ok(colors)
}
