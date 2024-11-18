// src/orbital.rs

use nalgebra_glm::Vec3;
use nalgebra_glm::Vec4;
use nalgebra_glm::Mat4;
use std::sync::Arc;
use fastnoise_lite::FastNoiseLite;
use crate::obj::Obj;
use crate::color::Color;
use crate::fragment::CelestialType;

#[derive(Clone, PartialEq)]
pub struct OrbitalElements {
    pub semi_major_axis: f32,              // a
    pub eccentricity: f32,                  // e
    pub inclination: f32,                   // i en radianes
    pub longitude_of_ascending_node: f32,  // Ω en radianes
    pub argument_of_periapsis: f32,         // ω en radianes
    pub mean_anomaly: f32,                  // M0 en radianes
    pub orbital_period: f32,                // T en segundos
}

impl OrbitalElements {
    pub fn new(
        semi_major_axis: f32,
        eccentricity: f32,
        inclination: f32,
        longitude_of_ascending_node: f32,
        argument_of_periapsis: f32,
        mean_anomaly: f32,
        orbital_period: f32,
    ) -> Self {
        OrbitalElements {
            semi_major_axis,
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
            mean_anomaly,
            orbital_period,
        }
    }
 /// Calcula la anomalía excéntrica usando el método de Newton-Raphson
 fn eccentric_anomaly(&self, M: f32) -> f32 {
    let mut E = M;
    for _ in 0..5 {
        E = E - (E - self.eccentricity * E.sin() - M) / (1.0 - self.eccentricity * E.cos());
    }
    E
}

/// Calcula la anomalía verdadera a partir de la excéntrica
fn true_anomaly(&self, E: f32) -> f32 {
    2.0 * (E / 2.0).tan().atan() // Simplificación para pequeñas excentricidades
}

/// Calcula la posición 3D en un tiempo dado
pub fn position_at(&self, time: f32) -> Vec3 {
    let n = 2.0 * std::f32::consts::PI / self.orbital_period; // Movimiento angular medio
    let M = self.mean_anomaly + n * time; // Anomalía media

    let E = self.eccentric_anomaly(M); // Anomalía excéntrica
    let ν = self.true_anomaly(E);       // Anomalía verdadera

    // Coordenadas en el plano orbital
    let r = self.semi_major_axis * (1.0 - self.eccentricity.powi(2)) / (1.0 + self.eccentricity * ν.cos());
    let x_orb = r * ν.cos();
    let y_orb = r * ν.sin();

    // Vector homogéneo
    let vec = Vec4::new(x_orb, y_orb, 0.0, 1.0);

    // Matriz de rotación: ω * i * Ω
    let rotation_matrix = nalgebra_glm::rotation(self.argument_of_periapsis, &Vec3::new(0.0, 0.0, 1.0))
        * nalgebra_glm::rotation(self.inclination, &Vec3::new(1.0, 0.0, 0.0))
        * nalgebra_glm::rotation(self.longitude_of_ascending_node, &Vec3::new(0.0, 1.0, 0.0));

    let rotated_vec = rotation_matrix * vec;

    // Extraer Vec3 ignorando el componente w
    Vec3::new(rotated_vec.x, rotated_vec.y, rotated_vec.z)
}
    pub fn position_at_anomaly(&self, true_anomaly: f32) -> Vec3 {
        // Cálculo de la distancia al sol
        let r = self.semi_major_axis * (1.0 - self.eccentricity.powi(2))
            / (1.0 + self.eccentricity * true_anomaly.cos());

        // Coordenadas en el plano orbital
        let x_orb = r * true_anomaly.cos();
        let y_orb = r * true_anomaly.sin();

        // Vector homogéneo
        let vec = Vec4::new(x_orb, y_orb, 0.0, 1.0);

        // Matriz de rotación
        let rotation_matrix =
            nalgebra_glm::rotation(self.longitude_of_ascending_node, &Vec3::new(0.0, 1.0, 0.0))
            * nalgebra_glm::rotation(self.inclination, &Vec3::new(1.0, 0.0, 0.0))
            * nalgebra_glm::rotation(self.argument_of_periapsis, &Vec3::new(0.0, 0.0, 1.0));

        let rotated_vec = rotation_matrix * vec;

        // Retornar la posición en 3D
        Vec3::new(rotated_vec.x, rotated_vec.y, rotated_vec.z)
    }
}

#[derive(Clone)]
pub struct CelestialBody {
    pub name: String,
    pub obj: Obj,
    pub shader_type: CelestialType,
    pub orbital_elements: OrbitalElements,
    pub scale: f32,
    pub rotation: Vec3,
    pub noise: Arc<FastNoiseLite>,
    pub noise_scale: f32,
    pub ocean_threshold: f32,
    pub continent_threshold: f32,
    pub mountain_threshold: f32,
    pub snow_threshold: f32,
    pub ring_inner_radius: f32,
    pub ring_outer_radius: f32,
    pub ring_color: Color,
    pub ring_opacity: f32,
    pub ring_frequency: f32,
    pub ring_wave_speed: f32,
    pub ring_rotation_matrix: Mat4,
    pub is_moon: bool,                // Nuevo campo
    pub orbiting_body_name: String,   // Nombre del cuerpo alrededor del cual orbita, si es una luna
}

impl CelestialBody {
    pub fn new(
        name: String,
        obj: Obj,
        shader_type: CelestialType,
        orbital_elements: OrbitalElements,
        scale: f32,
        rotation: Vec3,
        noise: Arc<FastNoiseLite>,
        noise_scale: f32,
        ocean_threshold: f32,
        continent_threshold: f32,
        mountain_threshold: f32,
        snow_threshold: f32,
        ring_inner_radius: f32,
        ring_outer_radius: f32,
        ring_color: Color,
        ring_opacity: f32,
        ring_frequency: f32,
        ring_wave_speed: f32,
        ring_rotation_matrix: Mat4,
        is_moon: bool,                // Añadido
        orbiting_body_name: String,   // Añadido
    ) -> Self {
        CelestialBody {
            name,
            obj,
            shader_type,
            orbital_elements,
            scale,
            rotation,
            noise,
            noise_scale,
            ocean_threshold,
            continent_threshold,
            mountain_threshold,
            snow_threshold,
            ring_inner_radius,
            ring_outer_radius,
            ring_color,
            ring_opacity,
            ring_frequency,
            ring_wave_speed,
            ring_rotation_matrix,
            is_moon,
            orbiting_body_name,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum CelestialBodyEnum {
    Star,
    Planet,
    GasGiant,
    Ringed,
    Rings,
    Planet2,
    Mars,
    Moon,
    Comet,
}

impl CelestialBodyEnum {
    pub fn to_celestial_type(&self) -> CelestialType {
        match self {
            CelestialBodyEnum::Star => CelestialType::Star,
            CelestialBodyEnum::Planet => CelestialType::Planet,
            CelestialBodyEnum::GasGiant => CelestialType::GasGiant,
            CelestialBodyEnum::Ringed => CelestialType::Ringed,
            CelestialBodyEnum::Rings => CelestialType::Rings,
            CelestialBodyEnum::Planet2 => CelestialType::Planet2,
            CelestialBodyEnum::Mars => CelestialType::Mars,
            CelestialBodyEnum::Moon => CelestialType::Moon,
            CelestialBodyEnum::Comet => CelestialType::Comet,
        }
    }
}

pub struct BodyManager {
    pub all_bodies: Vec<CelestialBody>,
    pub current_index: usize,
    pub zoom_level: f32,
}

impl BodyManager {
    pub fn new(
        star_obj: Obj,
        planet_obj: Obj,
        gas_giant_obj: Obj,
        ringed_obj: Obj,
        rings_obj: Obj,
        planet2_obj: Obj,
        mars_obj: Obj,
        moon_obj: Obj,
        comet_obj: Obj,
        noise_star: Arc<FastNoiseLite>,
        noise_planet: Arc<FastNoiseLite>,
        noise_gas_giant: Arc<FastNoiseLite>,
        noise_moon: Arc<FastNoiseLite>,
        noise_comet: Arc<FastNoiseLite>,
    ) -> Self {
        BodyManager {
            all_bodies: vec![
                // Estrella
                CelestialBody::new(
                    "Star".to_string(),
                    star_obj,
                    CelestialType::Star,
                    OrbitalElements::new(
                        0.0, // semi_major_axis (estrella en el centro)
                        0.0, // eccentricity
                        0.0, // inclination
                        0.0, // longitude_of_ascending_node
                        0.0, // argument_of_periapsis
                        0.0, // mean_anomaly
                        0.0, // orbital_period (no orbita)
                    ),
                    1.0, // scale
                    Vec3::new(0.0, 0.0, 0.0), // rotation
                    noise_star.clone(),
                    1.0,        // noise_scale
                    -0.6,       // ocean_threshold
                    0.65,       // continent_threshold
                    0.1,        // mountain_threshold
                    0.0,        // snow_threshold
                    0.0,        // ring_inner_radius
                    0.0,        // ring_outer_radius
                    Color::black(), // ring_color
                    0.0,        // ring_opacity
                    0.0,        // ring_frequency
                    0.0,        // ring_wave_speed
                    Mat4::identity(), // ring_rotation_matrix
                    false,      // is_moon
                    "".to_string(), // orbiting_body_name
                ),
                // Planeta
                CelestialBody::new(
                    "Planet".to_string(),
                    planet_obj,
                    CelestialType::Planet,
                    OrbitalElements::new(
                        5.0, // semi_major_axis
                        0.1, // eccentricity
                        5.0 * std::f32::consts::PI / 180.0, // inclinación en radianes
                        0.0, // longitude_of_ascending_node
                        0.0, // argument_of_periapsis
                        0.0, // mean_anomaly
                        30.0, // orbital_period (segundos)
                    ),
                    0.3, // scale
                    Vec3::new(0.0, 0.0, 0.0), // rotation
                    noise_planet.clone(),
                    3.0, // noise_scale
                    -0.038, // ocean_threshold
                    0.85, // continent_threshold
                    0.2, // mountain_threshold
                    0.05, // snow_threshold
                    0.0, // ring_inner_radius
                    0.0, // ring_outer_radius
                    Color::black(), // ring_color
                    0.0, // ring_opacity
                    0.0, // ring_frequency
                    0.0, // ring_wave_speed
                    Mat4::identity(), // ring_rotation_matrix
                    false,      // is_moon
                    "".to_string(), // orbiting_body_name
                ),
                // Gigante Gaseoso
                CelestialBody::new(
                    "GasGiant".to_string(),
                    gas_giant_obj,
                    CelestialType::GasGiant,
                    OrbitalElements::new(
                        8.0, // semi_major_axis
                        0.05, // eccentricity
                        10.0 * std::f32::consts::PI / 180.0, // inclinación
                        0.0,
                        0.0,
                        0.0,
                        65.0, // orbital_period
                    ),
                    0.5, // scale
                    Vec3::new(0.0, 0.0, 0.0), // rotation
                    noise_gas_giant.clone(),
                    15.0, // noise_scale
                    -0.6, // ocean_threshold
                    0.65, // continent_threshold
                    0.1, // mountain_threshold
                    0.0, // snow_threshold
                    0.0, // ring_inner_radius
                    0.0, // ring_outer_radius
                    Color::black(), // ring_color
                    0.0, // ring_opacity
                    0.0, // ring_frequency
                    0.0, // ring_wave_speed
                    Mat4::identity(), // ring_rotation_matrix
                    false,      // is_moon
                    "".to_string(), // orbiting_body_name
                ),
                // Planeta con Anillos
                CelestialBody::new(
                    "Ringed".to_string(),
                    ringed_obj,
                    CelestialType::Ringed,
                    OrbitalElements::new(
                        10.0, // semi_major_axis
                        0.02, // eccentricity
                        15.0 * std::f32::consts::PI / 180.0, // inclinación
                        0.0, // longitude_of_ascending_node
                        0.0, // argument_of_periapsis
                        0.0, // mean_anomaly
                        35.0, // orbital_period
                    ),
                    0.5, // scale
                    Vec3::new(0.0, 0.0, 0.0), // rotation
                    noise_gas_giant.clone(),
                    15.0, // noise_scale
                    -0.6, // ocean_threshold
                    0.65, // continent_threshold
                    0.1, // mountain_threshold
                    0.0, // snow_threshold
                    0.0, // ring_inner_radius
                    0.0, // ring_outer_radius
                    Color::black(), // ring_color
                    0.0, // ring_opacity
                    0.0, // ring_frequency
                    0.0, // ring_wave_speed
                    Mat4::identity(), // ring_rotation_matrix
                    false,      // is_moon
                    "".to_string(), // orbiting_body_name
                ),
                // Anillos
                CelestialBody::new(
                    "Rings".to_string(),
                    rings_obj,
                    CelestialType::Rings,
                    OrbitalElements::new(
                        10.0, // Same semi_major_axis as Ringed
                        0.02, // Same eccentricity
                        15.0 * std::f32::consts::PI / 180.0, // Same inclinación
                        0.0,
                        0.0,
                        0.0,
                        35.0, // orbital_period
                    ),
                    0.4, // scale (ajusta según el modelo de anillos)
                    Vec3::new(45.0, 0.0, 25.0), // rotation (ajusta para la inclinación de los anillos)
                    noise_gas_giant.clone(),
                    0.0, // noise_scale (no usado para anillos)
                    0.0, // ocean_threshold
                    0.0, // continent_threshold
                    0.0, // mountain_threshold
                    0.0, // snow_threshold
                    1.0, // ring_inner_radius
                    4.0, // ring_outer_radius
                    Color::new(200, 200, 200), // ring_color
                    0.7, // ring_opacity
                    15.0, // ring_frequency
                    0.5, // ring_wave_speed
                    nalgebra_glm::rotation(45.0_f32.to_radians(), &Vec3::new(1.0, 0.0, 0.0)), // ring_rotation_matrix
                    false,      // is_moon
                    "".to_string(), // orbiting_body_name
                ),
                // Planet2
                CelestialBody::new(
                    "Planet2".to_string(),
                    planet2_obj,
                    CelestialType::Planet2,
                    OrbitalElements::new(
                        6.0, // semi_major_axis
                        0.05, // eccentricity
                        7.0 * std::f32::consts::PI / 180.0, // inclinación
                        0.0, // longitude_of_ascending_node
                        0.0, // argument_of_periapsis
                        0.0, // mean_anomaly
                        40.0, // orbital_period
                    ),
                    0.3, // scale
                    Vec3::new(0.0, 0.0, 0.0), // rotation
                    noise_planet.clone(),
                    3.0, // noise_scale
                    -0.038, // ocean_threshold
                    0.85, // continent_threshold
                    0.2, // mountain_threshold
                    0.05, // snow_threshold
                    0.0, // ring_inner_radius
                    0.0, // ring_outer_radius
                    Color::black(), // ring_color
                    0.0, // ring_opacity
                    0.0, // ring_frequency
                    0.0, // ring_wave_speed
                    Mat4::identity(), // ring_rotation_matrix
                    false,      // is_moon
                    "".to_string(), // orbiting_body_name
                ),
                // Marte
                CelestialBody::new(
                    "Mars".to_string(),
                    mars_obj,
                    CelestialType::Mars,
                    OrbitalElements::new(
                        4.0, // semi_major_axis
                        0.08, // eccentricity
                        3.0 * std::f32::consts::PI / 180.0, // inclinación
                        0.0, // longitude_of_ascending_node
                        0.0, // argument_of_periapsis
                        0.0, // mean_anomaly
                        20.0, // orbital_period
                    ),
                    0.2, // scale
                    Vec3::new(0.0, 0.0, 0.0), // rotation
                    noise_planet.clone(),
                    6.0, // noise_scale
                    -0.038, // ocean_threshold
                    0.85, // continent_threshold
                    0.2, // mountain_threshold
                    0.05, // snow_threshold
                    0.0, // ring_inner_radius
                    0.0, // ring_outer_radius
                    Color::black(), // ring_color
                    0.0, // ring_opacity
                    0.0, // ring_frequency
                    0.0, // ring_wave_speed
                    Mat4::identity(), // ring_rotation_matrix
                    false,      // is_moon
                    "".to_string(), // orbiting_body_name
                ),
                // Luna
                CelestialBody::new(
                    "Moon".to_string(),
                    moon_obj,
                    CelestialType::Moon,
                    OrbitalElements::new(
                        0.7, // semi_major_axis
                        0.01, // eccentricity
                        5.0 * std::f32::consts::PI / 180.0, // inclinación
                        0.0, // longitude_of_ascending_node
                        0.0, // argument_of_periapsis
                        0.0, // mean_anomaly
                        7.0, // orbital_period
                    ),
                    0.06, // scale
                    Vec3::new(0.0, 0.0, 0.0), // rotation
                    noise_moon.clone(),
                    2.0, // noise_scale
                    -0.5, // ocean_threshold
                    0.6, // continent_threshold
                    0.2, // mountain_threshold
                    0.0, // snow_threshold
                    0.0, // ring_inner_radius
                    0.0, // ring_outer_radius
                    Color::black(), // ring_color
                    0.0, // ring_opacity
                    0.0, // ring_frequency
                    0.0, // ring_wave_speed
                    Mat4::identity(), // ring_rotation_matrix
                    true,       // is_moon
                    "Planet".to_string(), // orbiting_body_name
                ),
                // Cometa
                CelestialBody::new(
                    "Comet".to_string(),
                    comet_obj,
                    CelestialType::Comet,
                    OrbitalElements::new(
                        12.0, // semi_major_axis
                        0.7, // eccentricity
                        30.0 * std::f32::consts::PI / 180.0, // inclinación
                        0.0, // longitude_of_ascending_node
                        0.0, // argument_of_periapsis
                        0.0, // mean_anomaly
                        100.0, // orbital_period
                    ),
                    0.1, // scale
                    Vec3::new(0.0, 0.0, 0.0), // rotation
                    noise_comet.clone(),
                    7.0, // noise_scale
                    -0.6, // ocean_threshold
                    0.65, // continent_threshold
                    0.1, // mountain_threshold
                    0.0, // snow_threshold
                    0.0, // ring_inner_radius
                    0.0, // ring_outer_radius
                    Color::black(), // ring_color
                    0.0, // ring_opacity
                    0.0, // ring_frequency
                    0.0, // ring_wave_speed
                    Mat4::identity(), // ring_rotation_matrix
                    false,      // is_moon
                    "".to_string(), // orbiting_body_name
                ),
                
            ],
            current_index: 0,
            zoom_level: 50.0,
        }
    }

    pub fn get_body_position(&self, body: &CelestialBody, time: f32) -> Vec3 {
        let mut position = body.orbital_elements.position_at(time);

        // Si es una luna, sumar la posición del cuerpo que orbita
        if body.is_moon {
            if let Some(parent_body) = self.get_body_by_name(&body.orbiting_body_name) {
                let parent_position = self.get_body_position(parent_body, time);
                position += parent_position;
            }
        }

        position
    }

    pub fn get_body_by_name(&self, name: &str) -> Option<&CelestialBody> {
        self.all_bodies.iter().find(|body| body.name == name)
    }

    pub fn zoom_in(&mut self) {
        self.zoom_level = (self.zoom_level / 1.1).max(10.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom_level *= 1.1;
    }

    pub fn next(&mut self) {
        self.current_index = (self.current_index + 1) % self.all_bodies.len();
    }

    pub fn select(&mut self, index: usize) {
        if index < self.all_bodies.len() {
            self.current_index = index;
        }
    }
}
