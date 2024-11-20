// src/warp.rs

use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use nalgebra_glm::Mat4;

pub struct WarpEffect {
    duration: f32,          // Duración total del warp en segundos
    elapsed_time: f32,      // Tiempo transcurrido
    is_complete: bool,      // Indica si el warp ha finalizado
}

impl WarpEffect {
    pub fn new() -> Self {
        WarpEffect {
            duration: 2.0,    // Warp dura 2 segundos
            elapsed_time: 0.0,
            is_complete: false,
        }
    }

    pub fn render(&mut self, framebuffer: &mut Framebuffer, camera: &Camera, delta_time: f32) {
        if self.is_complete {
            return;
        }

        self.elapsed_time += delta_time;

        // Calcular progreso del warp (0.0 a 1.0)
        let progress = (self.elapsed_time / self.duration).min(1.0);

        // Aplicar efecto de distorsión basado en el progreso
        apply_warp_effect(framebuffer, progress);

        if self.elapsed_time >= self.duration {
            self.is_complete = true;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_complete
    }
}

fn apply_warp_effect(framebuffer: &mut Framebuffer, progress: f32) {
    // Ejemplo simple: aumentar la intensidad de los colores para simular el warp
    for pixel in framebuffer.buffer.iter_mut() {
        let r = ((pixel >> 16) & 0xFF) as f32;
        let g = ((pixel >> 8) & 0xFF) as f32;
        let b = (pixel & 0xFF) as f32;

        // Incrementar brillo basado en el progreso
        let factor = 1.0 + progress * 2.0;
        let r = (r * factor).min(255.0) as u32;
        let g = (g * factor).min(255.0) as u32;
        let b = (b * factor).min(255.0) as u32;

        *pixel = (r << 16) | (g << 8) | b;
    }
}
