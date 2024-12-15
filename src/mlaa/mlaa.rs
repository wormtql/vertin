use image::{Rgba, RgbaImage};

pub struct MLAAPrecompute;

impl MLAAPrecompute {
    fn f64_to_u8(f: f64) -> u8 {
        let f = f.max(0.0).min(1.0);

        (f * 255.0).round() as u8
    }

    fn fill_blank(tex: &mut RgbaImage, i: u32, j: u32) {
        for a in 0..9 {
            for b in 0..9 {
                let x = i * 9 + a;
                let y = j * 9 + b;
                tex.put_pixel(x, y, Rgba([0, 0, 0, 1]));
            }
        }
    }

    fn fill_01(tex: &mut RgbaImage, i: u32, j: u32, mode: usize) {
        for a in 0..9 {
            for b in 0..9 {
                let x = i * 9 + a;
                let y = j * 9 + b;

                let weight_top = (2 * a + 1) as f64 / (a + b + 1) as f64 * 0.25;
                let pix = Self::f64_to_u8(weight_top);

                if mode == 0 {
                    tex.put_pixel(x, y, Rgba([0, pix, 0, 255]));
                } else if mode == 1 {
                    tex.put_pixel(x, y, Rgba([pix, 0, 0, 255]));
                } else if mode == 2 {
                    tex.put_pixel(x, y, Rgba([pix, pix, 0, 255]));
                }
            }
        }
    }

    fn fill_10(tex: &mut RgbaImage, i: u32, j: u32, revert: bool) {
        for a in 0..9 {
            for b in 0..9 {
                let x = i * 9 + a;
                let y = j * 9 + b;

                let weight = (2 * b + 1) as f64 / (a + b + 1) as f64 * 0.25;
                let pix = Self::f64_to_u8(weight);

                if revert {
                    tex.put_pixel(x, y, Rgba([0, pix, 0, 255]));
                } else {
                    tex.put_pixel(x, y, Rgba([pix, 0, 0, 255]));
                }
            }
        }
    }

    fn fill_13(tex: &mut RgbaImage, i: u32, j: u32, revert: bool) {
        for a in 0..9 {
            for b in 0..9 {
                let x = i * 9 + a;
                let y = j * 9 + b;

                if b >= a + 1 {
                    let weight = 0.5 * (b - a) as f64 / (a + b + 1) as f64;
                    let pix = Self::f64_to_u8(weight);
                    if revert {
                        tex.put_pixel(x, y, Rgba([0, pix, 0, 255]));
                    } else {
                        tex.put_pixel(x, y, Rgba([pix, 0, 0, 255]));
                    }
                } else if b == a {
                    let weight = 1.0 / (16 * a + 8) as f64;
                    let pix = Self::f64_to_u8(weight);
                    tex.put_pixel(x, y, Rgba([pix, pix, 0, 255]));
                } else {
                    let weight = 0.5 * (a - b) as f64 / (a + b + 1) as f64;
                    let pix = Self::f64_to_u8(weight);
                    if revert {
                        tex.put_pixel(x, y, Rgba([pix, 0, 0, 255]));
                    } else {
                        tex.put_pixel(x, y, Rgba([0, pix, 0, 255]));
                    }
                }
            }
        }
    }

    pub fn precompute_lut() -> RgbaImage {
        let size = 5 * 9;
        let mut result = RgbaImage::new(size, size);

        for i in 0..5 {
            for j in 0..5 {
                if i == 2 || j == 2 {
                    Self::fill_blank(&mut result, i, j);
                } else if i == 0 && j == 0 {
                    Self::fill_blank(&mut result, i, j);
                } else if i == 0 && j == 1 {
                    Self::fill_01(&mut result, i, j, 1);
                } else if i == 0 && j == 3 {
                    Self::fill_01(&mut result, i, j, 0);
                } else if i == 0 && j == 4 {
                    // Self::fill_01(&mut result, i, j, 2);
                    Self::fill_blank(&mut result, i, j);
                } else if i == 1 && j == 0 {
                    Self::fill_10(&mut result, i, j, false);
                } else if i == 1 && j == 1 {
                    Self::fill_blank(&mut result, i, j);
                } else if i == 1 && j == 3 {
                    Self::fill_13(&mut result, i, j, false);
                } else if i == 1 && j == 4 {
                    Self::fill_13(&mut result, i, j, false);
                } else if i == 3 && j == 0 {
                    Self::fill_10(&mut result, i, j, true);
                } else if i == 3 && j == 1 {
                    Self::fill_13(&mut result, i, j, true);
                } else if i == 3 && j == 3 {
                    Self::fill_blank(&mut result, i, j);
                } else if i == 3 && j == 4 {
                    Self::fill_13(&mut result, i, j, true);
                } else if i == 4 && j == 0 {
                    Self::fill_blank(&mut result, i, j);
                } else if i == 4 && j == 1 {
                    Self::fill_13(&mut result, i, j, true);
                } else if i == 4 && j == 3 {
                    Self::fill_13(&mut result, i, j, false);
                } else if i == 4 && j == 4 {
                    Self::fill_blank(&mut result, i, j);
                }
            }
        }

        result
    }
}