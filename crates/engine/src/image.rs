use std::collections::HashMap;
use std::path::Path;

use crate::canvas::Canvas;
use crate::rectarea::RectArea;
use crate::settings::COLOR_COUNT;
use crate::tilemap::{Tile, Tilemap};
use crate::types::{Color, Rgb8};
use crate::utility::{parse_hex_string, simplify_string};
use crate::Pyxel;

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Vec<Color>>,
    palette: [Color; COLOR_COUNT as usize],
    self_rect: RectArea,
    clip_rect: RectArea,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let mut image = Image {
            width: width,
            height: height,
            data: vec![vec![0; width as usize]; height as usize],
            palette: [0; COLOR_COUNT as usize],
            self_rect: RectArea::new(0, 0, width, height),
            clip_rect: RectArea::new(0, 0, width, height),
        };

        image.pal_();

        image
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pal(&mut self, col1: Color, col2: Color) {
        self.palette[col1 as usize] = col2;
    }

    pub fn pal_(&mut self) {
        for i in 0..COLOR_COUNT {
            self.palette[i as usize] = i as Color;
        }
    }

    pub fn set(&mut self, x: i32, y: i32, data: &[&str]) {
        let width = data[0].len() as u32;
        let height = data.len() as u32;

        if width == 0 || height == 0 {
            return;
        }

        let mut tmp_image = Image::new(width, height);
        let tmp_data = tmp_image.data_mut();

        for i in 0..height {
            let data = simplify_string(data[i as usize]);

            for j in 0..width {
                if let Some(value) = parse_hex_string(&data[j as usize..j as usize + 1]) {
                    tmp_data[i as usize][j as usize] = value as Color;
                } else {
                    panic!("invalid image data");
                }
            }
        }

        self.blt(x, y, &tmp_image, 0, 0, width as i32, height as i32, None);
    }

    pub fn bltm(
        &mut self,
        x: i32,
        y: i32,
        src: &Tilemap,
        u: i32,
        v: i32,
        width: i32,
        height: i32,
        tile_key: Option<Tile>,
    ) {
        //
    }

    pub fn text(&mut self, pyxel: &Pyxel, x: i32, y: i32, text: &str, color: Color) {
        //
    }

    pub fn load(&mut self, x: i32, y: i32, filename: &str, color: &[Rgb8]) {
        let src_image = image::open(&Path::new(&filename)).unwrap().to_rgb8();
        let (width, height) = src_image.dimensions();
        let mut dst_image = Image::new(width, height);
        let dst_data = &mut dst_image.data;
        let mut color_table = HashMap::<(u8, u8, u8), Color>::new();

        for i in 0..height {
            for j in 0..width {
                let p = src_image.get_pixel(j, i);
                let src_rgb = (p[0], p[1], p[2]);

                if let Some(color) = color_table.get(&src_rgb) {
                    dst_data[i as usize][j as usize] = *color;
                } else {
                    let mut closest_color: Color = 0;
                    let mut closest_dist: f64 = f64::MAX;

                    for k in 0..=COLOR_COUNT {
                        let pal_color = color[k as usize];
                        let pal_rgb = (
                            ((pal_color >> 16) & 0xff) as u8,
                            ((pal_color >> 8) & 0xff) as u8,
                            (pal_color & 0xff) as u8,
                        );
                        let dist = Image::color_dist(src_rgb, pal_rgb);

                        if dist < closest_dist {
                            closest_color = k as Color;
                            closest_dist = dist;
                        }
                    }

                    color_table.insert(src_rgb, closest_color);
                    dst_data[i as usize][j as usize] = closest_color;
                }
            }
        }

        self.blt(x, y, &dst_image, 0, 0, width as i32, height as i32, None);
    }

    pub fn save(&self, filename: &str, scale: u32) {
        //
    }

    fn color_dist(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
        let (r1, g1, b1) = rgb1;
        let (r2, g2, b2) = rgb2;

        let dx = (r1 as f64 - r2 as f64) * 0.30;
        let dy = (g1 as f64 - g2 as f64) * 0.59;
        let dz = (b1 as f64 - b2 as f64) * 0.11;

        dx * dx + dy * dy + dz * dz
    }
}

impl Canvas<Color> for Image {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn data<'a>(&'a self) -> &'a Vec<Vec<Color>> {
        &self.data
    }

    fn data_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<Color>> {
        &mut self.data
    }

    fn _self_rect(&self) -> RectArea {
        self.self_rect
    }

    fn _clip_rect(&self) -> RectArea {
        self.clip_rect
    }

    fn _clip_rect_mut(&mut self) -> &mut RectArea {
        &mut self.clip_rect
    }

    fn _palette_value(&self, val: Color) -> Color {
        self.palette[val as usize]
    }
}
