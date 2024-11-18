// src/star.rs

use nalgebra_glm::{Vec3, Mat4};
use crate::color::Color;

pub struct Star {
    pub position: Vec3,     // Posición en el espacio 3D
    pub brightness: f32,    // Brillo de la estrella (0.0 a 1.0)
    pub color: Color,       // Color de la estrella
}

impl Star {
    pub fn new(position: Vec3, brightness: f32, color: Color) -> Self {
        Star {
            position,
            brightness: brightness.clamp(0.0, 1.0),
            color,
        }
    }
}
