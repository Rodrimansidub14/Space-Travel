// src/noise.rs

use fastnoise_lite::{FastNoiseLite, NoiseType, CellularDistanceFunction, FractalType};
use std::sync::Arc;

pub fn create_noise_star() -> Arc<FastNoiseLite> {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::Perlin)); // Ruido Perlin para variaciones suaves
    noise.set_frequency(Some(1.0)); // Alta frecuencia para detalles finos
    Arc::new(noise)
}

pub fn create_noise_planet() -> Arc<FastNoiseLite> {
    let mut noise = FastNoiseLite::with_seed(1338);
    noise.set_noise_type(Some(NoiseType::Perlin)); // Ruido Perlin para detalles medios
    noise.set_frequency(Some(0.35)); // Frecuencia media para detalles moderados
    Arc::new(noise)
}

pub fn create_noise_gas_giant() -> Arc<FastNoiseLite> {
    let mut noise = FastNoiseLite::with_seed(1637);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_frequency(Some(0.025)); // Ajusta según la densidad de detalles deseados

    // Configurar el fractal FBm para detalles adicionales
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(3));            // Detalle de octavas
    noise.set_fractal_lacunarity(Some(2.0));       // Separación de detalles
    noise.set_fractal_gain(Some(0.5));             // Ganancia para contrastes

    Arc::new(noise)
}

pub fn create_noise_moon() -> Arc<FastNoiseLite> {
    let mut noise = FastNoiseLite::with_seed(1340);
    noise.set_noise_type(Some(NoiseType::Perlin)); // Ruido Perlin para cráteres y detalles superficiales
    noise.set_frequency(Some(0.5)); // Alta frecuencia para detalles más finos
    Arc::new(noise)
}

pub fn create_noise_comet() -> Arc<FastNoiseLite> {
    let mut noise = FastNoiseLite::with_seed(1341);
    noise.set_noise_type(Some(NoiseType::Perlin)); // Ruido Perlin para patrones irregulares
    noise.set_frequency(Some(0.4)); // Frecuencia media
    Arc::new(noise)
}

pub fn create_noise_nebula() -> Arc<FastNoiseLite> {
    let mut noise = FastNoiseLite::with_seed(1342);
    noise.set_noise_type(Some(NoiseType::Cellular)); // Ruido Cellular para patrones nebulosos
    noise.set_cellular_distance_function(Some(CellularDistanceFunction::EuclideanSq));
    noise.set_frequency(Some(0.1)); // Muy baja frecuencia para grandes estructuras
    Arc::new(noise)
}
