use super::configure::SQUARE_SIZE;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

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
                match *user_context {
                    TextureColor::T0 => {
                        let r = colors[0][0];
                        let g = colors[0][1];
                        let b = colors[0][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T1 => {
                        let r = colors[1][0];
                        let g = colors[1][1];
                        let b = colors[1][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T2 => {
                        let r = colors[2][0];
                        let g = colors[2][1];
                        let b = colors[2][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T3 => {
                        let r = colors[3][0];
                        let g = colors[3][1];
                        let b = colors[3][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T4 => {
                        let r = colors[4][0];
                        let g = colors[4][1];
                        let b = colors[4][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T5 => {
                        let r = colors[5][0];
                        let g = colors[5][1];
                        let b = colors[5][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T6 => {
                        let r = colors[6][0];
                        let g = colors[6][1];
                        let b = colors[6][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T7 => {
                        let r = colors[7][0];
                        let g = colors[7][1];
                        let b = colors[7][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T8 => {
                        let r = colors[8][0];
                        let g = colors[8][1];
                        let b = colors[8][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T9 => {
                        let r = colors[9][0];
                        let g = colors[9][1];
                        let b = colors[9][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T10 => {
                        let r = colors[10][0];
                        let g = colors[10][1];
                        let b = colors[10][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T11 => {
                        let r = colors[11][0];
                        let g = colors[11][1];
                        let b = colors[11][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T12 => {
                        let r = colors[12][0];
                        let g = colors[12][1];
                        let b = colors[12][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T13 => {
                        let r = colors[13][0];
                        let g = colors[13][1];
                        let b = colors[13][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T14 => {
                        let r = colors[14][0];
                        let g = colors[14][1];
                        let b = colors[14][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T15 => {
                        let r = colors[15][0];
                        let g = colors[15][1];
                        let b = colors[15][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T16 => {
                        let r = colors[16][0];
                        let g = colors[16][1];
                        let b = colors[16][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T17 => {
                        let r = colors[17][0];
                        let g = colors[17][1];
                        let b = colors[17][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T18 => {
                        let r = colors[18][0];
                        let g = colors[18][1];
                        let b = colors[18][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T19 => {
                        let r = colors[19][0];
                        let g = colors[19][1];
                        let b = colors[19][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T20 => {
                        let r = colors[20][0];
                        let g = colors[20][1];
                        let b = colors[20][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T21 => {
                        let r = colors[21][0];
                        let g = colors[21][1];
                        let b = colors[21][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T22 => {
                        let r = colors[22][0];
                        let g = colors[22][1];
                        let b = colors[22][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T23 => {
                        let r = colors[23][0];
                        let g = colors[23][1];
                        let b = colors[23][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T24 => {
                        let r = colors[24][0];
                        let g = colors[24][1];
                        let b = colors[24][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T25 => {
                        let r = colors[25][0];
                        let g = colors[25][1];
                        let b = colors[25][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T26 => {
                        let r = colors[26][0];
                        let g = colors[26][1];
                        let b = colors[26][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T27 => {
                        let r = colors[27][0];
                        let g = colors[27][1];
                        let b = colors[27][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T28 => {
                        let r = colors[28][0];
                        let g = colors[28][1];
                        let b = colors[28][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T29 => {
                        let r = colors[29][0];
                        let g = colors[29][1];
                        let b = colors[29][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T30 => {
                        let r = colors[30][0];
                        let g = colors[30][1];
                        let b = colors[30][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T31 => {
                        let r = colors[31][0];
                        let g = colors[31][1];
                        let b = colors[31][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T32 => {
                        let r = colors[32][0];
                        let g = colors[32][1];
                        let b = colors[32][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T33 => {
                        let r = colors[33][0];
                        let g = colors[33][1];
                        let b = colors[33][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T34 => {
                        let r = colors[34][0];
                        let g = colors[34][1];
                        let b = colors[34][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T35 => {
                        let r = colors[35][0];
                        let g = colors[35][1];
                        let b = colors[35][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T36 => {
                        let r = colors[36][0];
                        let g = colors[36][1];
                        let b = colors[36][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T37 => {
                        let r = colors[37][0];
                        let g = colors[37][1];
                        let b = colors[37][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T38 => {
                        let r = colors[38][0];
                        let g = colors[38][1];
                        let b = colors[38][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T39 => {
                        let r = colors[39][0];
                        let g = colors[39][1];
                        let b = colors[39][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T40 => {
                        let r = colors[40][0];
                        let g = colors[40][1];
                        let b = colors[40][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T41 => {
                        let r = colors[41][0];
                        let g = colors[41][1];
                        let b = colors[41][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T42 => {
                        let r = colors[42][0];
                        let g = colors[42][1];
                        let b = colors[42][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T43 => {
                        let r = colors[43][0];
                        let g = colors[43][1];
                        let b = colors[43][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T44 => {
                        let r = colors[44][0];
                        let g = colors[44][1];
                        let b = colors[44][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T45 => {
                        let r = colors[45][0];
                        let g = colors[45][1];
                        let b = colors[45][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T46 => {
                        let r = colors[46][0];
                        let g = colors[46][1];
                        let b = colors[46][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T47 => {
                        let r = colors[47][0];
                        let g = colors[47][1];
                        let b = colors[47][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T48 => {
                        let r = colors[48][0];
                        let g = colors[48][1];
                        let b = colors[48][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T49 => {
                        let r = colors[49][0];
                        let g = colors[49][1];
                        let b = colors[49][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T50 => {
                        let r = colors[50][0];
                        let g = colors[50][1];
                        let b = colors[50][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T51 => {
                        let r = colors[51][0];
                        let g = colors[51][1];
                        let b = colors[51][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T52 => {
                        let r = colors[52][0];
                        let g = colors[52][1];
                        let b = colors[52][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T53 => {
                        let r = colors[53][0];
                        let g = colors[53][1];
                        let b = colors[53][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T54 => {
                        let r = colors[54][0];
                        let g = colors[54][1];
                        let b = colors[54][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::T55 => {
                        let r = colors[55][0];
                        let g = colors[55][1];
                        let b = colors[55][2];
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(r, g, b));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
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

pub fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>), String> {
    enum TextureColor {
        Zero,
        One,
        Two,
        Three,
    }
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

    {
        let textures = vec![
            (&mut square_texture1, TextureColor::Zero),
            (&mut square_texture2, TextureColor::One),
            (&mut square_texture3, TextureColor::Two),
            (&mut square_texture4, TextureColor::Three),
        ];
        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                match *user_context {
                    TextureColor::Zero => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::One => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(85, 85, 85));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::Two => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(170, 170, 170));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::Three => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(250, 250, 250));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                };
            })
            .map_err(|e| e.to_string())?;
    }
    Ok((
        square_texture1,
        square_texture2,
        square_texture3,
        square_texture4,
    ))
}
