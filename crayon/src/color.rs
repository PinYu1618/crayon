//! argb 8888
//!
//! Ref: redox-os/orbtk/utils/src/color.rs
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Color(u32);

impl Color {
    pub fn r(self) -> u8 {
        ((self.0 & 0x00FF_0000) >> 16) as u8
    }

    pub fn g(self) -> u8 {
        ((self.0 & 0x0000_FF00) >> 8) as u8
    }

    pub fn b(self) -> u8 {
        (self.0 & 0x0000_00FF) as u8
    }

    pub fn a(self) -> u8 {
        ((self.0 & 0xFF00_0000) >> 24) as u8
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color(0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub fn from_hsva(mut hue: f64, mut saturation: f64, mut value: f64, alpha: f64) -> Self {
        hue %= 360.0;
        saturation = saturation.max(0.0).min(1.0);
        value = value.max(0.0).min(1.0);
        let hh = hue / 60.0;
        let idx = hh.floor() as i32;
        let ff = hh.fract();
        let chroma = value * (1.0 - saturation);
        let second_component = value * (1.0 - (saturation * ff));
        let t = value * (1.0 - (saturation * (1.0 - ff)));
        let (r, g, b) = match idx {
            0 => (value, t, chroma),
            1 => (second_component, value, chroma),
            2 => (chroma, value, t),
            3 => (chroma, second_component, value),
            4 => (t, chroma, value),
            5 => (value, chroma, second_component),
            _ => unreachable!(),
        };
        Self::from_rgba(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (alpha * 255.0) as u8,
        )
    }

    pub fn from_hsla(mut hue: f64, mut saturation: f64, mut lightness: f64, alpha: f64) -> Self {
        hue %= 360.0;
        saturation = saturation.max(0.0).min(1.0);
        lightness = lightness.max(0.0).min(1.0);
        let hh = hue / 60.0;
        let idx = hh.floor() as i32;
        let chroma = (1.0 - ((2.0 * lightness) - 1.0).abs()) * saturation;
        let second_component = chroma * (1.0 - (((idx % 2) as f64) - 1.0).abs());
        let (mut r, mut g, mut b) = match idx {
            0 => (chroma, second_component, 0.0),
            1 => (second_component, chroma, 0.0),
            2 => (0.0, chroma, second_component),
            3 => (0.0, second_component, chroma),
            4 => (second_component, 0.0, chroma),
            5 => (chroma, 0.0, second_component),
            _ => unreachable!(),
        };
        let adjustment = lightness - chroma / 2.0;
        r += adjustment;
        g += adjustment;
        b += adjustment;
        Self::from_rgba(
            (r.min(1.0) * 255.0) as u8,
            (g.min(1.0) * 255.0) as u8,
            (b.min(1.0) * 255.0) as u8,
            (alpha * 255.0) as u8,
        )
    }

    pub fn interpolate(start_color: Color, end_color: Color, scale: f64) -> Color {
        let r = Color::interp(start_color.r(), end_color.r(), scale);
        let g = Color::interp(start_color.g(), end_color.g(), scale);
        let b = Color::interp(start_color.b(), end_color.b(), scale);
        let a = Color::interp(start_color.a(), end_color.a(), scale);
        Color::from_rgba(r, g, b, a)
    }

    fn interp(start_color: u8, end_color: u8, scale: f64) -> u8 {
        (end_color as f64 - start_color as f64).mul_add(scale, start_color as f64) as u8
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.a() == other.a() && self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}