
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const NONE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const ORANGE: Self = Self { r: 1.0, g: 0.5, b: 0.0, a: 1.0 };
    pub const PURPLE: Self = Self { r: 0.5, g: 0.0, b: 0.5, a: 1.0 };
    pub const GRAY: Self = Self { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
    pub const LIGHT_GRAY: Self = Self { r: 0.75, g: 0.75, b: 0.75, a: 1.0 };
    pub const DARK_GRAY: Self = Self { r: 0.25, g: 0.25, b: 0.25, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    pub fn random() -> Self {
        Self {
            r: rand::random::<f32>(),
            g: rand::random::<f32>(),
            b: rand::random::<f32>(),
            a: 1.0,
        }
    }

}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [((self.r * 255.0) as u8), ((self.g * 255.0) as u8), ((self.b * 255.0) as u8), ((self.a * 255.0) as u8)]
    }
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }
}