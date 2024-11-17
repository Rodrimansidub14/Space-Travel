use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    pub emissive_buffer: Vec<u32>, // Nuevo buffer para emisivos
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            emissive_buffer: vec![0; width * height], // Inicializar
            background_color: 0x000000,
            current_color: 0x000000,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
        for emissive_pixel in self.emissive_buffer.iter_mut() {
            *emissive_pixel = 0x000000;
        }
    }
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color, depth: f32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = y as usize * self.width + x as usize;
            if self.zbuffer[index] > depth {
                self.buffer[index] = color.to_hex();
                self.zbuffer[index] = depth;
            }
        }
    }
    pub fn point(&mut self, x: usize, y: usize, depth: f32, emissive: bool) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                if emissive {
                    self.emissive_buffer[index] = self.current_color;
                }
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }
    
}
pub fn post_process(framebuffer: &mut Framebuffer) {
    for i in 0..framebuffer.buffer.len() {
        let emissive = framebuffer.emissive_buffer[i];
        let final_color = blend_add(framebuffer.buffer[i], emissive);
        framebuffer.buffer[i] = final_color;
    }
}

// Función para mezclar colores usando blend add
pub fn blend_add(base: u32, emissive: u32) -> u32 {
    let r = ((base >> 16) & 0xFF).saturating_add((emissive >> 16) & 0xFF);
    let g = ((base >> 8) & 0xFF).saturating_add((emissive >> 8) & 0xFF);
    let b = (base & 0xFF).saturating_add(emissive & 0xFF);
    (r << 16) | (g << 8) | b
}