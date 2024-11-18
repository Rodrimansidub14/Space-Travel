// src/camera.rs

use nalgebra_glm::{Vec3, rotate_vec3,Mat3, lerp};
use std::f32::consts::PI;
/// Estructura que representa la cámara en el espacio 3D

pub enum CameraMode {
    Free,               // Modo libre: orbita, drag, zoom
    Fixed {             // Modo fijo en un planeta
        target_index: usize, // Índice del planeta objetivo
        angle: f32,          // Ángulo de rotación alrededor del planeta
    },
}
pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool,
    target: Option<Vec3>, // Añadido para seguimiento

}

impl Camera {
    /// Crea una nueva cámara
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
            target:None,
            
        }
    }
    pub fn follow(&mut self, target_position: Vec3) {
        self.target = Some(target_position);
        self.update_view();
    }

    pub fn stop_following(&mut self) {
        self.target = None;
    }

    pub fn update_view(&mut self) {
        if let Some(target_pos) = self.target {
            self.center = target_pos;
            // Ajusta la posición de la cámara según sea necesario
            self.eye = target_pos + Vec3::new(0.0, 0.0, 10.0); // Ejemplo de desplazamiento
        }
    }
    /// Cambia la base de la cámara (no utilizado actualmente)
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let rotated = 
            vector.x * right +
            vector.y * up +
            -vector.z * forward;

        rotated.normalize()
    }

    /// Orbita la cámara alrededor del punto central con cambios en yaw y pitch
 /// Orbita la cámara alrededor del punto central con cambios en delta_x y delta_y
 pub fn orbit(&mut self, delta_x: f32, delta_y: f32) {
    let direction = self.eye - self.center;
    let horizontal_axis = self.up.normalize();
    let vertical_axis = direction.cross(&horizontal_axis).normalize();

    // Rotar el vector de dirección alrededor del eje horizontal
    let rotated_direction_horizontal = rotate_vec3(&direction, delta_x, &horizontal_axis);

    // Rotar el vector de dirección alrededor del eje vertical
    let final_direction = rotate_vec3(&rotated_direction_horizontal, delta_y, &vertical_axis);

    self.eye = self.center + final_direction;
    self.up = rotate_vec3(&self.up, delta_y, &vertical_axis);

    self.has_changed = true;
}

    /// Realiza un zoom in o out moviendo la cámara hacia o desde el punto central
    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalize();
        self.eye += direction * delta;
        self.has_changed = true;
    }
    
    /// Mueve el punto central de la cámara (no es necesario en la funcionalidad actual)
    pub fn move_center(&mut self, direction: Vec3) {
        let radius_vector = self.center - self.eye;
        let radius = radius_vector.magnitude();

        let angle_x = direction.x * 0.05; // Ajusta este factor para controlar la velocidad de rotación
        let angle_y = direction.y * 0.05;

        let rotated = rotate_vec3(&radius_vector, angle_x, &Vec3::new(0.0, 1.0, 0.0));

        let right = rotated.cross(&self.up).normalize();
        let final_rotated = rotate_vec3(&rotated, angle_y, &right);

        self.center = self.eye + final_rotated.normalize() * radius;
        self.has_changed = true;
    }

    /// Verifica si la cámara ha cambiado
    pub fn check_if_changed(&mut self) -> bool {
        if self.has_changed {
            self.has_changed = false;
            true
        } else {
            false
        }
    }
    pub fn interpolate_to(&mut self, target_eye: Vec3, target_center: Vec3, t: f32) {
        self.eye = lerp(&self.eye, &target_eye, t);
        self.center = lerp(&self.center, &target_center, t);
        self.has_changed = true;
    }

}
