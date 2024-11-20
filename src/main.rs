use nalgebra_glm::{Vec3, Vec4, Mat4};
use minifb::{Key, Window, WindowOptions, MouseButton, MouseMode};
use std::time::Duration;
use std::sync::Arc;

// Importa tus módulos aquí
mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;
mod star;    // Añade esta línea
mod uniforms;
mod renderer;
mod orbital;
mod noise; // Añadido
mod stars; // Añade esta línea

use stars::StarField; // Y esta línea para usar StarField

use framebuffer::{Framebuffer, post_process};
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use color::Color;
use fastnoise_lite::{FastNoiseLite, NoiseType, CellularDistanceFunction, FractalType};
use renderer::render;
use fragment::CelestialType;
use uniforms::Uniforms;
use orbital::{OrbitalElements, CelestialBody, CelestialBodyEnum, BodyManager};
use noise::{create_noise_star, create_noise_planet, create_noise_gas_giant, create_noise_moon, create_noise_comet, create_noise_nebula}; // Añadido
// Importa ParticleSystem si es necesario
const SIZE_SCALE: f32 = 1.0;
const DISTANCE_SCALE: f32 = 1.0;
// Función para crear la matriz de modelo
fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let scaled_scale = scale * SIZE_SCALE;
    let scaled_translation = translation * DISTANCE_SCALE;

    // Matriz de escala
    let scale_matrix = Mat4::new_scaling(scaled_scale);

    // Matrices de rotación
    let rotation_x = Mat4::from_euler_angles(rotation.x, 0.0, 0.0);
    let rotation_y = Mat4::from_euler_angles(0.0, rotation.y, 0.0);
    let rotation_z = Mat4::from_euler_angles(0.0, 0.0, rotation.z);

    let rotation_matrix = rotation_z * rotation_y * rotation_x;

    // Matriz de traslación basada en la posición orbital
    let translation_matrix = Mat4::new_translation(&scaled_translation);

    // Combinamos todas las matrices
    translation_matrix * rotation_matrix * scale_matrix
}


// Función para crear la matriz de vista
fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    nalgebra_glm::look_at(&eye, &center, &up)
}

// Función para crear la matriz de perspectiva
fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * std::f32::consts::PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    nalgebra_glm::perspective(fov, aspect_ratio, near, far)
}

// Función para crear una matriz de rotación a partir de ángulos de Euler (en grados)
fn create_rotation_matrix(pitch: f32, yaw: f32, roll: f32) -> Mat4 {
    Mat4::from_euler_angles(
        pitch * std::f32::consts::PI / 180.0,
        yaw * std::f32::consts::PI / 180.0,
        roll * std::f32::consts::PI / 180.0,
    )
}

// Función para crear la matriz de viewport
fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

// Función para aplicar el post-procesamiento
fn generate_orbital_path(orbital_elements: &OrbitalElements) -> Vec<Vec3> {
    let mut path = Vec::new();
    let steps = 360; // Número de puntos para definir la órbita

    for i in 0..steps {
        let true_anomaly = 2.0 * std::f32::consts::PI * i as f32 / steps as f32;
        let position = orbital_elements.position_at_anomaly(true_anomaly);
        path.push(position);
    }

    path
}

fn render_orbital_points(
    framebuffer: &mut Framebuffer,
    path: &Vec<Vec3>,
    view_matrix: &Mat4,
    projection_matrix: &Mat4,
    viewport_matrix: &Mat4,
) {
    let color = Color::new(255, 255, 255); // Color blanco para los puntos

    for point in path {
        // Transformar el punto al espacio de clip
        let world_pos = Vec4::new(point.x, point.y, point.z, 1.0);
        let clip_space = projection_matrix * view_matrix * world_pos;

        // Normalizar por w
        if clip_space.w != 0.0 {
            let ndc = Vec3::new(
                clip_space.x / clip_space.w,
                clip_space.y / clip_space.w,
                clip_space.z / clip_space.w,
            );

            // Transformar al espacio de pantalla
            let screen_space = viewport_matrix * Vec4::new(ndc.x, ndc.y, ndc.z, 1.0);

            // Dibujar el punto con profundidad
            framebuffer.set_pixel(
                screen_space.x.round() as i32,
                screen_space.y.round() as i32,
                color,
                ndc.z, // Usar la coordenada z en NDC como profundidad
            );
        }
    }
}



// Función para manejar la entrada del usuario
// src/main.rs

fn handle_input(
    window: &Window,
    camera: &mut Camera,
    body_manager: &mut BodyManager,
    time: f32,
    is_dragging: &mut bool,
    last_mouse_pos: &mut (f32, f32),
) {
    let rotation_speed = 0.005;
    let zoom_speed = 0.35; // Ajusta este valor para controlar la sensibilidad del zoom

    // Manejar el arrastre del mouse para rotar la cámara
    if window.get_mouse_down(MouseButton::Left) {
        let mouse_pos = window.get_mouse_pos(MouseMode::Discard).unwrap_or(*last_mouse_pos);
        if !*is_dragging {
            *is_dragging = true;
        } else {
            let delta_x = (mouse_pos.0 - last_mouse_pos.0) * rotation_speed;
            let delta_y = (mouse_pos.1 - last_mouse_pos.1) * rotation_speed;

            // Asegurarse de que los ángulos están en radianes
            let delta_x_rad = delta_x; // rotation_speed ya está en radianes
            let delta_y_rad = delta_y;

            camera.orbit(delta_x_rad, delta_y_rad);
        }
        *last_mouse_pos = mouse_pos;
    } else {
        *is_dragging = false;
    }

    // Manejar el zoom con la rueda del mouse
    if let Some(scroll) = window.get_scroll_wheel() {
        camera.zoom(scroll.1 * zoom_speed); // Multiplica por zoom_speed para ajustar la sensibilidad
    }

    // Manejar otras entradas de teclado
    let rotation_speed_keyboard = 0.05;

    // Cambiar al siguiente cuerpo celeste al presionar 'N'


    // Rotación de la cámara con las teclas de flecha
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0, rotation_speed_keyboard * 0.01); // Ajusta el factor según necesidad
    }
    if window.is_key_down(Key::Down) {
        camera.orbit(0.0, -rotation_speed_keyboard * 0.01);
    }
    if window.is_key_down(Key::Left) {
        camera.orbit(-rotation_speed_keyboard * 0.01, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(rotation_speed_keyboard * 0.01, 0.0);
    }

    // Selección de cuerpos celestes con teclas numéricas
    for num in 1..=9 {
        let key = match num {
            1 => Key::Key1,
            2 => Key::Key2,
            3 => Key::Key3,
            4 => Key::Key4,
            5 => Key::Key5,
            6 => Key::Key6,
            7 => Key::Key7,
            8 => Key::Key8,
            9 => Key::Key9,
            _ => continue,
        };

        if window.is_key_down(key) {
            body_manager.select(num - 1);
            let selected_body = &body_manager.all_bodies[body_manager.current_index];
            let position = body_manager.get_body_position(selected_body, time);
            camera.follow(position);
        }
    }

    // Detener seguimiento al presionar 'S'
    if window.is_key_down(Key::S) {
        camera.stop_following();
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16); // Aproximadamente 60 FPS

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Animated Fragment Shader",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000);

    let mut is_dragging = false;
    let mut last_mouse_pos = (0.0, 0.0);
    // Parámetros de la cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0), // Eye
        Vec3::new(0.0, 0.0, 0.0),  // Center
        Vec3::new(0.0, 1.0, 0.0),  // Up
    );
    let star_field = StarField::new(1000, 100.0); // 1000 estrellas dentro de un radio de 100 unidades

    // Crear generadores de ruido separados para cada cuerpo celeste
    let noise_star = create_noise_star();
    let noise_planet = create_noise_planet();
    let noise_gas_giant = create_noise_gas_giant();
    let noise_moon = create_noise_moon();
    let noise_comet = create_noise_comet();
    let noise_nebula = create_noise_nebula();

    // Cargar modelos
    let star_obj = Obj::load("assets/models/planet.obj").expect("Failed to load star.obj");
    let star_vertex_array = star_obj.get_vertex_array();

    let planet_obj = Obj::load("assets/models/planet.obj").expect("Failed to load planet.obj");
    let planet_vertex_array = planet_obj.get_vertex_array();

    let gas_giant_obj = Obj::load("assets/models/planet.obj").expect("Failed to load gas_giant.obj");
    let gas_giant_vertex_array = gas_giant_obj.get_vertex_array();

    let ringed_obj = Obj::load("assets/models/planet.obj").expect("Failed to load ringed.obj");
    let ringed_vertex_array = ringed_obj.get_vertex_array();

    let rings_obj = Obj::load("assets/models/rings2.obj").expect("Failed to load rings.obj");
    let rings_vertex_array = rings_obj.get_vertex_array();
    let planet_obj = Obj::load("assets/models/planet.obj").expect("Failed to load planet.obj");
    let planet_vertex_array = planet_obj.get_vertex_array();
    let moon_obj = Obj::load("assets/models/planet.obj").expect("Failed to load moon.obj");
    let moon_vertex_array = moon_obj.get_vertex_array();
    let planet2_obj = Obj::load("assets/models/planet.obj").expect("Failed to load planet.obj");
    let planet2_vertex_array = planet2_obj.get_vertex_array();
    let mars_obj = Obj::load("assets/models/planet.obj").expect("Failed to load planet.obj");
    let mars_vertex_array = mars_obj.get_vertex_array();
    let comet_obj = Obj::load("assets/models/planet.obj").expect("Failed to load comet.obj");
    let comet_vertex_array = comet_obj.get_vertex_array();
    let mut time = 0.0; // Usar f32 para mayor precisión en cálculos de tiempo
    let spaceship_obj = Obj::load("assets/models/ship.obj").expect("Failed to load ship.obj");
    let spaceship_vertex_array = spaceship_obj.get_vertex_array();

    // Inicializar BodyManager
    let mut body_manager = BodyManager::new(
        star_obj,
        planet_obj,
        gas_giant_obj,
        ringed_obj,
        rings_obj,
        planet2_obj,
        mars_obj,
        moon_obj,
        comet_obj,
        noise_star,
        noise_planet,
        noise_gas_giant,
        noise_moon,
        noise_comet,
    );

    // src/main.rs



while window.is_open() && !window.is_key_down(Key::Escape) {
    // Actualizar el tiempo
    time += 0.016; // Aproximadamente 60 FPS

    // Manejar entradas
    handle_input(
        &window,
        &mut camera,
        &mut body_manager,
        time,
        &mut is_dragging,
        &mut last_mouse_pos,
    );

    // Actualizar la posición de la nave

    // Limpiar el framebuffer
    framebuffer.clear();

    // Crear matrices de transformación
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);


    // Definir la dirección de la luz
    let light_direction = Vec3::new(1.0, 1.0, 1.0).normalize();

    // Renderizar solo la estrella
    let star = &body_manager.all_bodies[0]; // Asumiendo que la estrella es el primer elemento
    let model_matrix = create_model_matrix(Vec3::zeros(), star.scale, star.rotation);
    let uniforms = Uniforms::new(
        model_matrix,
        view_matrix,
        projection_matrix,
        viewport_matrix,
        time,
        star.noise.clone(),
        light_direction,
        star.noise_scale,
        star.ocean_threshold,
        star.continent_threshold,
        star.mountain_threshold,
        star.snow_threshold,
        star.ring_inner_radius,
        star.ring_outer_radius,
        star.ring_color,
        star.ring_opacity,
        star.ring_frequency,
        star.ring_wave_speed,
        star.ring_rotation_matrix,
    );


    for body in &body_manager.all_bodies {
        // Obtener la posición actual del cuerpo
        let position = body_manager.get_body_position(body, time);
        
        // Calcular la dirección de la luz (desde el planeta hacia el sol)
        let light_direction = (-position).normalize(); // Suponiendo que el sol está en (0,0,0)
        
        // Crear la matriz de modelo con la posición y rotación propia
        let model_matrix = create_model_matrix(position, body.scale, body.rotation);
        
        // Crear las uniformes necesarias para el shader
        let uniforms = Uniforms::new(
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            body.noise.clone(),
            light_direction, // Actualizado dinámicamente
            body.noise_scale,
            body.ocean_threshold,
            body.continent_threshold,
            body.mountain_threshold,
            body.snow_threshold,
            body.ring_inner_radius,
            body.ring_outer_radius,
            body.ring_color,
            body.ring_opacity,
            body.ring_frequency,
            body.ring_wave_speed,
            body.ring_rotation_matrix,
        );
        
        // Renderizar el cuerpo celeste
        render(&mut framebuffer, &uniforms, &body.obj.get_vertex_array(), body.shader_type);
    }
 

    // Renderizar las líneas orbitales como puntos
// Renderizar las líneas orbitales como puntos, excluyendo la estrella
    for body in &body_manager.all_bodies {
        if body.name != "Star" {
            let orbital_path = generate_orbital_path(&body.orbital_elements);
            render_orbital_points(&mut framebuffer, &orbital_path, &view_matrix, &projection_matrix, &viewport_matrix);
        }
    }
    render(&mut framebuffer, &uniforms, &star_vertex_array, CelestialType::Star);

 
          // Renderizar las líneas orbitales como puntos, excluyendo la estrella
          for body in &body_manager.all_bodies {
              if body.name != "Star" {
                  let orbital_path = generate_orbital_path(&body.orbital_elements);
                  render_orbital_points(&mut framebuffer, &orbital_path, &view_matrix, &projection_matrix, &viewport_matrix);
              }
          }
          star_field.render(&mut framebuffer, &camera, &projection_matrix, &viewport_matrix);

          // Post-Procesamiento para Emisión (si es necesario)
          post_process(&mut framebuffer);

          // Actualizar la ventana con el framebuffer
          window
              .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
              .unwrap();
  
          // Control de la tasa de frames
          std::thread::sleep(frame_delay);
      }
  }