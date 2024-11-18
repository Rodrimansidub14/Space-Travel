// src/stars.rs

use nalgebra::{Vector3, Point3, Matrix4, Vector4};
use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use crate::color::Color;
use crate::star::Star;
use rand::Rng;

pub struct StarField {
    pub stars: Vec<Star>,
}

impl StarField {
    /// Genera un campo de estrellas aleatorio
    pub fn new(num_stars: usize, radius: f32) -> Self {
        let mut stars = Vec::with_capacity(num_stars);
        let mut rng = rand::thread_rng();

        for _ in 0..num_stars {
            // Generar una posición aleatoria dentro de una esfera
            let theta: f32 = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
            let phi: f32 = rng.gen_range(0.0..std::f32::consts::PI);
            let r: f32 = rng.gen_range(0.0..radius);

            let x = r * theta.cos() * phi.sin();
            let y = r * theta.sin() * phi.sin();
            let z = r * phi.cos();

            // Generar brillo y color aleatorio
            let brightness: f32 = rng.gen_range(0.5..1.0); // Evitar estrellas muy débiles
            let color = Color::new(
                rng.gen_range(200..256) as u8,
                rng.gen_range(200..256) as u8,
                rng.gen_range(200..256) as u8,
            );

            stars.push(Star::new(Vector3::new(x, y, z), brightness, color));
        }

        StarField { stars }
    }

    /// Renderiza las estrellas en el framebuffer
    pub fn render(&self, framebuffer: &mut Framebuffer, camera: &Camera, projection_matrix: &Matrix4<f32>, viewport_matrix: &Matrix4<f32>) {
        for star in &self.stars {
            // Transformar la posición de la estrella al espacio de cámara
            let model_matrix: Matrix4<f32> = Matrix4::identity(); // Sin transformación adicional
    
            // Crear matriz de vista-proyección usando look_at_rh
            let view_matrix = Matrix4::look_at_rh(
                &Point3::new(camera.eye.x, camera.eye.y, camera.eye.z),
                &Point3::new(camera.center.x, camera.center.y, camera.center.z),
                &Vector3::from(camera.up),
            );
    
            let view_projection = projection_matrix * view_matrix;
    
            // Convertir Vec3 a Point3
            let position_point = Point3::new(star.position.x, star.position.y, star.position.z);
    
            // Convertir Point3 a Vec4
            let position_vec4 = position_point.to_homogeneous(); // Añade el componente w=1.0
    
            let clip_space = view_projection * position_vec4;
    
            // Ignorar estrellas detrás del plano cercano o lejano
            if clip_space.w <= 0.0 {
                continue;
            }
    
            // Convertir a NDC
            let ndc = Vector3::new(
                clip_space.x / clip_space.w,
                clip_space.y / clip_space.w,
                clip_space.z / clip_space.w,
            );
    
            // Convertir a coordenadas de pantalla
            let screen_space = viewport_matrix * Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
            let x = screen_space.x.round() as i32;
            let y = screen_space.y.round() as i32;
    
            // Establecer el pixel si está dentro de la pantalla
            if x >= 0 && x < framebuffer.width as i32 && y >= 0 && y < framebuffer.height as i32 {
                // Aplicar el brillo al color
                let color = star.color * star.brightness;
                framebuffer.set_pixel(x, y, color, ndc.z);
            }
        }
    }
    
}
