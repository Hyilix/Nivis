const MAX_COLOR_VAL: u8 = 255;

// Conversion functions
#[inline]
pub fn normal_to_u8(val: f32) -> u8 {
    (val.clamp(0.0, 0.1) * MAX_COLOR_VAL as f32).round() as u8
}

#[inline]
pub fn u8_to_normal(val: u8) -> f32 {
    val as f32 / MAX_COLOR_VAL as f32
}

// Classic representation for color (RGBA: 0-255)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0, MAX_COLOR_VAL)
    }
}

// Boilerplate getters and setters
impl Color {
    // Standard 0-255 range
    pub fn get_r(&self) -> u8 {
        self.r
    }

    pub fn get_g(&self) -> u8 {
        self.g
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    // 0.0 .. 1.0 float range
    pub fn get_normalized_r(&self) -> f32 {
        u8_to_normal(self.r)
    }

    pub fn get_normalized_g(&self) -> f32 {
        u8_to_normal(self.g)
    }

    pub fn get_normalized_b(&self) -> f32 {
        u8_to_normal(self.b)
    }

    pub fn get_normalized_a(&self) -> f32 {
        u8_to_normal(self.a)
    }

    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }

    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    pub fn set_normalized_r(&mut self, r: f32) {
        self.r = normal_to_u8(r);
    }

    pub fn set_normalized_g(&mut self, g: f32) {
        self.g = normal_to_u8(g);
    }

    pub fn set_normalized_b(&mut self, b: f32) {
        self.b = normal_to_u8(b);
    }

    pub fn set_normalized_a(&mut self, a: f32) {
        self.a = normal_to_u8(a);
    }
}

impl Color {
    // Standard way of creating new color (0-255)
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_normalized(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(normal_to_u8(r), normal_to_u8(g), normal_to_u8(b), normal_to_u8(a))
    }

    // From softbuffer 0xAARRGGBB
    pub fn from_u32(val: u32) -> Self {
        Self::new(
            ((val >> 16) & 0xFF) as u8,
            ((val >> 8) & 0xFF) as u8,
            (val & 0xFF) as u8,
            ((val >> 24) & 0xFF) as u8
        )
    }

    // To softbuffer 0xAARRGGBB
    pub fn to_u32(&self) -> u32 {
        (self.a as u32) << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b) as u32
    }

    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub fn to_normalized_tuple(&self) -> (f32, f32, f32, f32) {
        (u8_to_normal(self.r),
         u8_to_normal(self.g),
         u8_to_normal(self.b),
         u8_to_normal(self.a))
    }
}
